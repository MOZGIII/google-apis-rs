// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

use google_customsearch1::{api, Error, oauth2, client::chrono, FieldMask};


use google_clis_common as client;

use client::{InvalidOptionsError, CLIError, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::error::Error as StdError;
use std::str::FromStr;

use serde_json as json;
use clap::ArgMatches;
use http::Uri;
use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service;

enum DoitError {
    IoError(String, io::Error),
    ApiError(Error),
}

struct Engine<'n, S> {
    opt: ArgMatches<'n>,
    hub: api::CustomSearchAPI<S>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n, S> Engine<'n, S>
where
    S: tower_service::Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn _cse_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cse().list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start" => {
                    call = call.start(        value.map(|v| arg_from_str(v, err, "start", "uint32")).unwrap_or(0));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "site-search-filter" => {
                    call = call.site_search_filter(value.unwrap_or(""));
                },
                "site-search" => {
                    call = call.site_search(value.unwrap_or(""));
                },
                "search-type" => {
                    call = call.search_type(value.unwrap_or(""));
                },
                "safe" => {
                    call = call.safe(value.unwrap_or(""));
                },
                "rights" => {
                    call = call.rights(value.unwrap_or(""));
                },
                "related-site" => {
                    call = call.related_site(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "or-terms" => {
                    call = call.or_terms(value.unwrap_or(""));
                },
                "num" => {
                    call = call.num(        value.map(|v| arg_from_str(v, err, "num", "int32")).unwrap_or(-0));
                },
                "lr" => {
                    call = call.lr(value.unwrap_or(""));
                },
                "low-range" => {
                    call = call.low_range(value.unwrap_or(""));
                },
                "link-site" => {
                    call = call.link_site(value.unwrap_or(""));
                },
                "img-type" => {
                    call = call.img_type(value.unwrap_or(""));
                },
                "img-size" => {
                    call = call.img_size(value.unwrap_or(""));
                },
                "img-dominant-color" => {
                    call = call.img_dominant_color(value.unwrap_or(""));
                },
                "img-color-type" => {
                    call = call.img_color_type(value.unwrap_or(""));
                },
                "hq" => {
                    call = call.hq(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "high-range" => {
                    call = call.high_range(value.unwrap_or(""));
                },
                "googlehost" => {
                    call = call.googlehost(value.unwrap_or(""));
                },
                "gl" => {
                    call = call.gl(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "file-type" => {
                    call = call.file_type(value.unwrap_or(""));
                },
                "exclude-terms" => {
                    call = call.exclude_terms(value.unwrap_or(""));
                },
                "exact-terms" => {
                    call = call.exact_terms(value.unwrap_or(""));
                },
                "date-restrict" => {
                    call = call.date_restrict(value.unwrap_or(""));
                },
                "cx" => {
                    call = call.cx(value.unwrap_or(""));
                },
                "cr" => {
                    call = call.cr(value.unwrap_or(""));
                },
                "c2coff" => {
                    call = call.c2coff(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["c2coff", "cr", "cx", "date-restrict", "exact-terms", "exclude-terms", "file-type", "filter", "gl", "googlehost", "high-range", "hl", "hq", "img-color-type", "img-dominant-color", "img-size", "img-type", "link-site", "low-range", "lr", "num", "or-terms", "q", "related-site", "rights", "safe", "search-type", "site-search", "site-search-filter", "sort", "start"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _cse_siterestrict_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.cse().siterestrict_list();
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "start" => {
                    call = call.start(        value.map(|v| arg_from_str(v, err, "start", "uint32")).unwrap_or(0));
                },
                "sort" => {
                    call = call.sort(value.unwrap_or(""));
                },
                "site-search-filter" => {
                    call = call.site_search_filter(value.unwrap_or(""));
                },
                "site-search" => {
                    call = call.site_search(value.unwrap_or(""));
                },
                "search-type" => {
                    call = call.search_type(value.unwrap_or(""));
                },
                "safe" => {
                    call = call.safe(value.unwrap_or(""));
                },
                "rights" => {
                    call = call.rights(value.unwrap_or(""));
                },
                "related-site" => {
                    call = call.related_site(value.unwrap_or(""));
                },
                "q" => {
                    call = call.q(value.unwrap_or(""));
                },
                "or-terms" => {
                    call = call.or_terms(value.unwrap_or(""));
                },
                "num" => {
                    call = call.num(        value.map(|v| arg_from_str(v, err, "num", "int32")).unwrap_or(-0));
                },
                "lr" => {
                    call = call.lr(value.unwrap_or(""));
                },
                "low-range" => {
                    call = call.low_range(value.unwrap_or(""));
                },
                "link-site" => {
                    call = call.link_site(value.unwrap_or(""));
                },
                "img-type" => {
                    call = call.img_type(value.unwrap_or(""));
                },
                "img-size" => {
                    call = call.img_size(value.unwrap_or(""));
                },
                "img-dominant-color" => {
                    call = call.img_dominant_color(value.unwrap_or(""));
                },
                "img-color-type" => {
                    call = call.img_color_type(value.unwrap_or(""));
                },
                "hq" => {
                    call = call.hq(value.unwrap_or(""));
                },
                "hl" => {
                    call = call.hl(value.unwrap_or(""));
                },
                "high-range" => {
                    call = call.high_range(value.unwrap_or(""));
                },
                "googlehost" => {
                    call = call.googlehost(value.unwrap_or(""));
                },
                "gl" => {
                    call = call.gl(value.unwrap_or(""));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                "file-type" => {
                    call = call.file_type(value.unwrap_or(""));
                },
                "exclude-terms" => {
                    call = call.exclude_terms(value.unwrap_or(""));
                },
                "exact-terms" => {
                    call = call.exact_terms(value.unwrap_or(""));
                },
                "date-restrict" => {
                    call = call.date_restrict(value.unwrap_or(""));
                },
                "cx" => {
                    call = call.cx(value.unwrap_or(""));
                },
                "cr" => {
                    call = call.cr(value.unwrap_or(""));
                },
                "c2coff" => {
                    call = call.c2coff(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["c2coff", "cr", "cx", "date-restrict", "exact-terms", "exclude-terms", "file-type", "filter", "gl", "googlehost", "high-range", "hl", "hq", "img-color-type", "img-dominant-color", "img-size", "img-type", "link-site", "low-range", "lr", "num", "or-terms", "q", "related-site", "rights", "safe", "search-type", "site-search", "site-search-filter", "sort", "start"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    async fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("cse", Some(opt)) => {
                match opt.subcommand() {
                    ("list", Some(opt)) => {
                        call_result = self._cse_list(opt, dry_run, &mut err).await;
                    },
                    ("siterestrict-list", Some(opt)) => {
                        call_result = self._cse_siterestrict_list(opt, dry_run, &mut err).await;
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("cse".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    async fn new(opt: ArgMatches<'n>, connector: S) -> Result<Engine<'n, S>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match client::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match client::application_secret_from_directory(&config_dir, "customsearch1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let client = hyper::Client::builder().build(connector);

        let auth = oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            client.clone(),
        ).persist_tokens_to_disk(format!("{}/customsearch1", config_dir)).build().await.unwrap();

        let engine = Engine {
            opt: opt,
            hub: api::CustomSearchAPI::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "callback", "fields", "key", "oauth-token", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
                ]
        };

        match engine._doit(true).await {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    async fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false).await {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("cse", "methods: 'list' and 'siterestrict-list'", vec![
            ("list",
                    Some(r##"Returns metadata about the search performed, metadata about the engine used for the search, and the search results."##),
                    "Details at http://byron.github.io/google-apis-rs/google_customsearch1_cli/cse_list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("siterestrict-list",
                    Some(r##"Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns."##),
                    "Details at http://byron.github.io/google-apis-rs/google_customsearch1_cli/cse_siterestrict-list",
                  vec![
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("customsearch1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("5.0.2+20230123")
           .about("Searches over a website or collection of websites")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_customsearch1_cli")
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Debug print all errors")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("adebug");
    let connector = hyper_rustls::HttpsConnectorBuilder::new().with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    match Engine::new(matches, connector).await {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit().await {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}
