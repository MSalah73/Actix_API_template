# Actix API template
## Basic actix based skeleton with few niceties
[![build status](https://github.com/Msalah73/actix_api_template/actions/workflows/general-ci-pipeline.yml/badge.svg?branch=master)](https://github.com/Msalah73/actix_api_template/actions/workflows/general-ci-pipeline.yml/)
[![Coverage Status](https://coveralls.io/repos/github/MSalah73/actix_api_template/badge.svg?branch=master)](https://coveralls.io/github/MSalah73/actix_api_template?branch=master)
[![dependency status](https://deps.rs/repo/github/Msalah73/actix_api_template/status.svg)](https://deps.rs/repo/github/Msalah73/actix_api_template)

The template has essential features:

- Decent telemetry coverage  -- Compatible with OpenTelemtry
- Native tokio based integration test -- can be easily replaced with you favorite integration framework
- basic Github action workflows
- Dockerfile with caching
- Tiny docker image - roughly 4 MB
- basic api file structure with heatlh_check and an example endpoint with logging
- highly configurable

## Note
This repository is a fully featured template to help me bootstrap my rust projects. I learned most of the style and ideas from [Zero2Prod](https://www.zero2prod.com/).
