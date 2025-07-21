// For more information on what these properties do, see shared/src/args.rs
export interface CmdArgs {
    thread_count: number;
    ffmpeg_options?: string | null;
    input?: string[] | null;
    file_list?: string | null;
    overwrite: boolean;
    verbose: boolean;
    delete: boolean;
    eta: boolean;
    output: string;
}

export enum LogSeverity {
    ERROR = 'error',
    INFO = 'info'
}