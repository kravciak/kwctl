use clap::{
    builder::PossibleValuesParser, crate_authors, crate_description, crate_name, crate_version,
    Arg, ArgAction, Command,
};
use itertools::Itertools;
use lazy_static::lazy_static;
use policy_evaluator::burrego;

lazy_static! {
    static ref VERSION_AND_BUILTINS: String = {
        let builtins: String = burrego::get_builtins()
            .keys()
            .sorted()
            .map(|builtin| format!("  - {builtin}"))
            .join("\n");

        format!(
            "{}\n\nOpen Policy Agent/Gatekeeper implemented builtins:\n{}",
            crate_version!(),
            builtins,
        )
    };
}

pub fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::new("verbose").short('v').help("Increase verbosity"))
        .subcommand(
            Command::new("policies")
                .about("Lists all downloaded policies")
        )
        .subcommand(
            Command::new("pull")
                .about("Pulls a Kubewarden policy from a given URI")
                .arg(
                    Arg::new("docker-config-json-path")
                    .long("docker-config-json-path")
                    .value_name("DOCKER_CONFIG")
                    .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
                .arg(
                    Arg::new("sources-path")
                    .long("sources-path")
                    .value_name("PATH")
                    .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("verification-config-path")
                    .long("verification-config-path")
                    .value_name("PATH")
                    .help("YAML file holding verification config information (signatures, public keys...)")
                )
                .arg(
                    Arg::new("verification-key")
                    .short('k')
                    .long("verification-key")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to key used to verify the policy. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("fulcio-cert-path")
                    .long("fulcio-cert-path")
                    .action(ArgAction::Append)
                    .value_name("PATH")
                    .help("Path to the Fulcio certificate. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("rekor-public-key-path")
                    .long("rekor-public-key-path")
                    .value_name("PATH")
                    .help("Path to the Rekor public key")
                )
                .arg(
                    Arg::new("verification-annotation")
                    .short('a')
                    .long("verification-annotation")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("KEY=VALUE")
                    .help("Annotation in key=value format. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("cert-email")
                    .long("cert-email")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected email in Fulcio certificate")
                )
                .arg(
                    Arg::new("cert-oidc-issuer")
                    .long("cert-oidc-issuer")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected OIDC issuer in Fulcio certificates")
                )
                .arg(
                    Arg::new("github-owner")
                    .long("github-owner")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub owner expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("github-repo")
                    .long("github-repo")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub repository expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("output-path")
                    .short('o')
                    .long("output-path")
                    .value_name("PATH")
                    .help("Output file. If not provided will be downloaded to the Kubewarden store")
                )
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI. Supported schemes: registry://, https://, file://")
                )
        )
        .subcommand(
            Command::new("verify")
                .about("Verify a Kubewarden policy from a given URI using Sigstore")
                .arg(
                    Arg::new("docker-config-json-path")
                    .long("docker-config-json-path")
                    .value_name("PATH")
                    .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
                .arg(
                    Arg::new("sources-path")
                    .long("sources-path")
                    .value_name("PATH")
                    .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("verification-config-path")
                    .long("verification-config-path")
                    .value_name("PATH")
                    .help("YAML file holding verification config information (signatures, public keys...)")
                )
                .arg(
                    Arg::new("verification-key")
                    .short('k')
                    .long("verification-key")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to key used to verify the policy. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("fulcio-cert-path")
                    .long("fulcio-cert-path")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to the Fulcio certificate. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("rekor-public-key-path")
                    .long("rekor-public-key-path")
                    .value_name("PATH")
                    .help("Path to the Rekor public key")
                )
                .arg(
                    Arg::new("verification-annotation")
                    .short('a')
                    .long("verification-annotation")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("KEY=VALUE")
                    .help("Annotation in key=value format. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("cert-email")
                    .long("cert-email")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected email in Fulcio certificate")
                )
                .arg(
                    Arg::new("cert-oidc-issuer")
                    .long("cert-oidc-issuer")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected OIDC issuer in Fulcio certificates")
                )
                .arg(
                    Arg::new("github-owner")
                    .long("github-owner")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub owner expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("github-repo")
                    .long("github-repo")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub repository expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI. Supported schemes: registry://")
                )
        )
        .subcommand(
            Command::new("push")
                .about("Pushes a Kubewarden policy to an OCI registry")
                .arg(
                    Arg::new("docker-config-json-path")
                    .long("docker-config-json-path")
                    .value_name("PATH")
                    .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
                .arg(
                    Arg::new("sources-path")
                    .long("sources-path")
                    .value_name("PATH")
                    .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("force")
                    .short('f')
                    .long("force")
                    .help("Push also a policy that is not annotated")
                )
                .arg(
                    Arg::new("output")
                    .long("output")
                    .short('o')
                    .value_name("PATH")
                    .value_parser(PossibleValuesParser::new(["text", "json"]))
                    .default_value("text")
                    .help("Output format")
                )
               .arg(
                    Arg::new("policy")
                        .required(true)
                        .index(1)
                        .help("Policy to push. Can be the path to a local file, or a policy URI")
                )
               .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(2)
                        .help("Policy URI. Supported schemes: registry://")
                )
        )
        .subcommand(
            Command::new("rm")
                .about("Removes a Kubewarden policy from the store")
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI")
                )
        )
        .subcommand(
            Command::new("run")
                .about("Runs a Kubewarden policy from a given URI")
                .arg(
                    Arg::new("docker-config-json-path")
                    .long("docker-config-json-path")
                    .value_name("PATH")
                    .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
                .arg(
                    Arg::new("sources-path")
                    .long("sources-path")
                    .value_name("PATH")
                    .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("verification-config-path")
                    .long("verification-config-path")
                    .value_name("PATH")
                    .help("YAML file holding verification config information (signatures, public keys...)")
                )
                .arg(
                    Arg::new("request-path")
                    .long("request-path")
                    .short('r')
                    .value_name("PATH")
                    .required(true)
                    .help("File containing the Kubernetes admission request object in JSON format")
                )
                .arg(
                    Arg::new("settings-path")
                    .long("settings-path")
                    .short('s')
                    .value_name("PATH")
                    .help("File containing the settings for this policy")
                )
                .arg(
                    Arg::new("settings-json")
                    .long("settings-json")
                    .value_name("VALUE")
                    .help("JSON string containing the settings for this policy")
                )
                .arg(
                    Arg::new("verification-key")
                    .short('k')
                    .long("verification-key")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to key used to verify the policy. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("fulcio-cert-path")
                    .long("fulcio-cert-path")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to the Fulcio certificate. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("rekor-public-key-path")
                    .long("rekor-public-key-path")
                    .value_name("PATH")
                    .help("Path to the Rekor public key")
                )
                .arg(
                    Arg::new("verification-annotation")
                    .short('a')
                    .long("verification-annotation")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("KEY=VALUE")
                    .help("Annotation in key=value format. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("cert-email")
                    .long("cert-email")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected email in Fulcio certificate")
                )
                .arg(
                    Arg::new("cert-oidc-issuer")
                    .long("cert-oidc-issuer")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected OIDC issuer in Fulcio certificates")
                )
                .arg(
                    Arg::new("github-owner")
                    .long("github-owner")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub owner expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("github-repo")
                    .long("github-repo")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub repository expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("execution-mode")
                    .long("execution-mode")
                    .short('e')
                    .value_name("MODE")
                    .value_parser(PossibleValuesParser::new(["opa","gatekeeper", "kubewarden"]))
                    .help("The runtime to use to execute this policy")
                )
                .arg(
                    Arg::new("disable-wasmtime-cache")
                    .long("disable-wasmtime-cache")
                    .help("Turn off usage of wasmtime cache")
                )
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI. Supported schemes: registry://, https://, file://. If schema is omitted, file:// is assumed, rooted on the current directory")
                )
        )
        .subcommand(
            Command::new("annotate")
                .about("Add Kubewarden metadata to a WebAssembly module")
                .arg(
                    Arg::new("metadata-path")
                    .long("metadata-path")
                    .short('m')
                    .required(true)
                    .value_name("PATH")
                    .help("File containing the metadata")
                )
                .arg(
                    Arg::new("output-path")
                    .long("output-path")
                    .short('o')
                    .required(true)
                    .value_name("PATH")
                    .help("Output file")
                )
                .arg(
                    Arg::new("wasm-path")
                    .required(true)
                    .index(1)
                    .help("Path to WebAssembly module to be annotated")
                )
        )
        .subcommand(
            Command::new("inspect")
                .about("Inspect Kubewarden policy")
                .arg(
                    Arg::new("output")
                    .long("output")
                    .short('o')
                    .value_name("FORMAT")
                    .value_parser(PossibleValuesParser::new(["yaml"]))
                    .help("Output format")
                )
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI. Supported schemes: registry://, https://, file://")
                )
                .arg(
                    Arg::new("sources-path")
                        .long("sources-path")
                        .value_name("PATH")
                        .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("docker-config-json-path")
                        .long("docker-config-json-path")
                        .value_name("PATH")
                        .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
        )
        .subcommand(
            Command::new("scaffold")
                .about("Scaffold a Kubernetes resource or configuration file")
                .subcommand_required(true)
                .subcommand(
                    Command::new("verification-config")
                        .about("Output a default Sigstore verification configuration file")
                )
                .subcommand(
                    Command::new("manifest")
                        .about("Output a Kubernetes resource manifest")
                        .arg(
                            Arg::new("settings-path")
                            .long("settings-path")
                            .short('s')
                            .value_name("PATH")
                            .help("File containing the settings for this policy")
                        )
                        .arg(
                            Arg::new("settings-json")
                            .long("settings-json")
                            .value_name("VALUE")
                            .help("JSON string containing the settings for this policy")
                        )
                        .arg(
                            Arg::new("type")
                            .long("type")
                            .short('t')
                            .required(true)
                            .value_name("VALUE")
                            .value_parser(PossibleValuesParser::new(["ClusterAdmissionPolicy", "AdmissionPolicy"]))
                            .help("Kubewarden Custom Resource type")
                        )
                        .arg(
                            Arg::new("uri")
                                .required(true)
                                .index(1)
                                .help("Policy URI. Supported schemes: registry://, https://, file://")
                        )
                        .arg(
                            Arg::new("title")
                                .long("title")
                                .value_name("VALUE")
                                .help("Policy title")
                        )
                )
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    Arg::new("shell")
                    .long("shell")
                    .short('s')
                    .value_name("VALUE")
                    .required(true)
                    .value_parser(PossibleValuesParser::new(["bash", "fish", "zsh", "elvish", "powershell"]))
                    .help("Shell type")
                )
        )
        .subcommand(
            Command::new("digest")
                .about("Fetch digest from the OCI manifest of a policy")
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI")
                )
                .arg(
                    Arg::new("sources-path")
                        .long("sources-path")
                        .value_name("PATH")
                        .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("docker-config-json-path")
                        .long("docker-config-json-path")
                        .value_name("PATH")
                        .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
        )
        .subcommand(
            Command::new("bench")
                .about("Benchmarks a Kubewarden policy from a given URI")
                .arg(
                    Arg::new("measurement_time")
                    .long("measurement-time")
                    .number_of_values(1)
                    .value_name("SECONDS")
                    .help("How long the bench ‘should’ run, num_samples is prioritized so benching will take longer to be able to collect num_samples if the code to be benched is slower than this time limit allowed")
                )
                .arg(
                    Arg::new("num_resamples")
                    .long("num-resamples")
                    .number_of_values(1)
                    .value_name("NUM")
                    .help("How many resamples should be done")
                )
                .arg(
                    Arg::new("num_samples")
                    .long("num-samples")
                    .number_of_values(1)
                    .value_name("NUM")
                    .help("How many resamples should be done. Recommended at least 50, above 100 doesn’t seem to yield a significantly different result")
                )
                .arg(
                    Arg::new("warm_up_time")
                    .long("warm-up-time")
                    .number_of_values(1)
                    .value_name("SECONDS")
                    .help("How long the bench should warm up")
                )
                .arg(
                    Arg::new("dump_results_to_disk")
                    .long("dump-results-to-disk")
                    .help("Puts results in target/tiny-bench/label/.. if target can be found. used for comparing previous runs")
                )

                // The next ones are exactly like the `run` args
                .arg(
                    Arg::new("docker-config-json-path")
                    .long("docker-config-json-path")
                    .value_name("PATH")
                    .help("Path to a Docker config.json-like path. Can be used to indicate registry authentication details")
                )
                .arg(
                    Arg::new("sources-path")
                    .long("sources-path")
                    .value_name("PATH")
                    .help("YAML file holding source information (https, registry insecure hosts, custom CA's...)")
                )
                .arg(
                    Arg::new("verification-config-path")
                    .long("verification-config-path")
                    .value_name("PATH")
                    .help("YAML file holding verification config information (signatures, public keys...)")
                )
                .arg(
                    Arg::new("request-path")
                    .long("request-path")
                    .short('r')
                    .value_name("PATH")
                    .required(true)
                    .help("File containing the Kubernetes admission request object in JSON format")
                )
                .arg(
                    Arg::new("settings-path")
                    .long("settings-path")
                    .short('s')
                    .value_name("PATH")
                    .help("File containing the settings for this policy")
                )
                .arg(
                    Arg::new("settings-json")
                    .long("settings-json")
                    .value_name("VALUE")
                    .help("JSON string containing the settings for this policy")
                )
                .arg(
                    Arg::new("verification-key")
                    .short('k')
                    .long("verification-key")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to key used to verify the policy. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("fulcio-cert-path")
                    .long("fulcio-cert-path")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("PATH")
                    .help("Path to the Fulcio certificate. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("rekor-public-key-path")
                    .long("rekor-public-key-path")
                    .value_name("PATH")
                    .help("Path to the Rekor public key")
                )
                .arg(
                    Arg::new("verification-annotation")
                    .short('a')
                    .long("verification-annotation")
                    .action(ArgAction::Append)
                    .number_of_values(1)
                    .value_name("KEY=VALUE")
                    .help("Annotation in key=value format. Can be repeated multiple times")
                )
                .arg(
                    Arg::new("cert-email")
                    .long("cert-email")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected email in Fulcio certificate")
                )
                .arg(
                    Arg::new("cert-oidc-issuer")
                    .long("cert-oidc-issuer")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("Expected OIDC issuer in Fulcio certificates")
                )
                .arg(
                    Arg::new("github-owner")
                    .long("github-owner")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub owner expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("github-repo")
                    .long("github-repo")
                    .number_of_values(1)
                    .value_name("VALUE")
                    .help("GitHub repository expected in the certificates generated in CD pipelines")
                )
                .arg(
                    Arg::new("execution-mode")
                    .long("execution-mode")
                    .short('e')
                    .value_name("MODE")
                    .value_parser(PossibleValuesParser::new(["opa","gatekeeper", "kubewarden"]))
                    .help("The runtime to use to execute this policy")
                )
                .arg(
                    Arg::new("disable-wasmtime-cache")
                    .long("disable-wasmtime-cache")
                    .help("Turn off usage of wasmtime cache")
                )
                .arg(
                    Arg::new("uri")
                        .required(true)
                        .index(1)
                        .help("Policy URI. Supported schemes: registry://, https://, file://. If schema is omitted, file:// is assumed, rooted on the current directory")
                )
        )
        .subcommand(
            Command::new("save")
                .about("save policies to a tar.gz file")
                .arg(
                    Arg::new("policies")
                        .num_args(1..)
                        .required(true)
                        .help("list of policies to save")
                )
                .arg(
                    Arg::new("output")
                    .long("output")
                    .short('o')
                    .required(true)
                    .value_name("FILE")
                    .help("path where the file will be stored")
                )

        )
        .subcommand(
            Command::new("load")
                .about("load policies from a tar.gz file")
                .arg(
                    Arg::new("input")
                        .long("input")
                        .required(true)
                        .help("load policies from tarball")
                )
        )
        .long_version(VERSION_AND_BUILTINS.as_str())
        .subcommand_required(true)
        .arg_required_else_help(true)
}
