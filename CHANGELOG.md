# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## 0.6.0 (12. july, 2023)
### Changed
- Updated ODBC-Api to 0.57.0

## 0.5.0 (31. March, 2023)
### Changed
- Updated ODBC-Api to 0.55.0
- Replaced Lazy_static with Once_Cell

## 0.4.0 (26. November, 2022)
### Changed
- Updated ODBC-Api to 0.52.2
- Updated Licenses to add Apache and Zlib.
- Removed Service
- Removed Layer
- Removed uneeded Depenencies
- Depreciated the block macro in regards to newest ODBC change adding Async execute_polling
- Removed FromRequest
- Updated Readme to use Axum 0.6 State
- Updated to Axum 0.6

### Added
- FromRequestParts for ODBCConnectionManager

## 0.3.1 (26. July, 2022)
### Fixed
- button links not being correct.

## 0.3.0 (26. July, 2022)
### Fixed
- Documentation not building.

### Changed
- updated dependencies

### Added
- Direct link to odbc-api so you dont need to import it yourself.

## 0.2.0 (14. June, 2022)
### Changed
- updated odbc-api to 0.44.2.
- removed unwrap() and return Result for require.

## 0.1.0 (23. May, 2022)
### Added
- Initial release.
