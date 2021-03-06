/*
 * Copyright 2019 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use crate::error::ConfigurationError;

#[derive(Debug)]
pub struct GridConfig {
    validator_endpoint: String,
    rest_api_endpoint: String,
    database_url: String,
}

impl GridConfig {
    pub fn validator_endpoint(&self) -> &str {
        &self.validator_endpoint
    }

    pub fn rest_api_endpoint(&self) -> &str {
        &self.rest_api_endpoint
    }
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

pub struct GridConfigBuilder {
    validator_endpoint: Option<String>,
    rest_api_endpoint: Option<String>,
    database_url: Option<String>,
}

impl Default for GridConfigBuilder {
    fn default() -> Self {
        Self {
            validator_endpoint: Some("tcp://127.0.0.1:4004".to_owned()),
            rest_api_endpoint: Some("127.0.0.1:8080".to_owned()),
            database_url: Some("postgres://grid:grid_example@localhost/grid".to_owned()),
        }
    }
}

impl GridConfigBuilder {
    pub fn with_cli_args(&mut self, matches: &clap::ArgMatches<'_>) -> Self {
        Self {
            validator_endpoint: matches
                .value_of("connect")
                .map(|c| {
                    let mut connect = ToOwned::to_owned(c);
                    if !connect.contains("tcp://") {
                        connect.insert_str(0, "tcp://");
                    }
                    connect
                })
                .or_else(|| self.validator_endpoint.take()),

            rest_api_endpoint: matches
                .value_of("bind")
                .map(ToOwned::to_owned)
                .or_else(|| self.rest_api_endpoint.take()),

            database_url: matches
                .value_of("database_url")
                .map(ToOwned::to_owned)
                .or_else(|| self.database_url.take()),
        }
    }

    pub fn build(mut self) -> Result<GridConfig, ConfigurationError> {
        Ok(GridConfig {
            validator_endpoint: self
                .validator_endpoint
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("validator_endpoint".to_owned()))?,
            rest_api_endpoint: self
                .rest_api_endpoint
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("rest_api_endpoint".to_owned()))?,
            database_url: self
                .database_url
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("database_url".to_owned()))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_with_args() {
        let matches = clap::App::new("testapp")
            .arg(clap::Arg::with_name("connect").short("C").takes_value(true))
            .arg(clap::Arg::with_name("bind").short("b").takes_value(true))
            .get_matches_from(vec![
                "testapp",
                "-C",
                "validator:4004",
                "-b",
                "rest_api:8080",
            ]);

        let config = GridConfigBuilder::default()
            .with_cli_args(&matches)
            .build()
            .expect("Unable to build configuration");

        assert_eq!("tcp://validator:4004", config.validator_endpoint());
        assert_eq!("rest_api:8080", config.rest_api_endpoint());
    }

    #[test]
    fn build_with_missing_args() {
        let matches = clap::App::new("testapp")
            .arg(clap::Arg::with_name("connect").short("C").takes_value(true))
            .arg(clap::Arg::with_name("bind").short("b").takes_value(true))
            .get_matches_from(vec!["testapp"]);

        let config = GridConfigBuilder::default()
            .with_cli_args(&matches)
            .build()
            .expect("Unable to build configuration");

        assert_eq!("tcp://127.0.0.1:4004", config.validator_endpoint());
        assert_eq!("127.0.0.1:8080", config.rest_api_endpoint());
    }
}
