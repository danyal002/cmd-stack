export enum CliPrintStyle {
    All = "All",
    CommandsOnly = "CommandsOnly",
};

export enum ApplicationTheme {
    Dark = "Dark",
    Light = "Light",
    System = "System"
};

export type SettingsConfig = {
    cli_print_style: CliPrintStyle,
    cli_display_limit: number,
    param_string_length_min: number,
    param_string_length_max: number,
    param_int_range_min: number,
    param_int_range_max: number,
    application_theme: ApplicationTheme
};
