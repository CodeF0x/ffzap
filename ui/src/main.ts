import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { CmdArgs } from './models';
import { listen } from '@tauri-apps/api/event';

enum LogSeverity {
    ERROR = 'error',
    INFO = 'info'
}

document.addEventListener('DOMContentLoaded', () => {
    let allFiles: string[] | null = null;
    let filesList: string | null = null;
    let totalFiles: number = 0;
    let doneFiles: number = 0;

    const tabButtons: NodeListOf<Element> = document.querySelectorAll('.tab-btn');
    const tabContents: NodeListOf<Element> = document.querySelectorAll('.tab-content');

    tabButtons.forEach((button: Element) => {
        button.addEventListener('click', () => {
            const targetTab: string | null = button.getAttribute('data-tab');

            tabButtons.forEach((btn: Element) => btn.classList.remove('active'));
            tabContents.forEach((content: Element) => content.classList.remove('active'));

            button.classList.add('active');
            const targetContent: HTMLElement | null = document.getElementById(`${targetTab}-tab`);
            if (targetContent) {
                targetContent.classList.add('active');
            }
        });
    });

    document.getElementById('browse-files-btn')!.addEventListener('click', async () => {
        const files: string[] | null = await open({
            multiple: true,
            directory: false,
        });

        allFiles = files;
        filesList = null;

        updateFileCount(allFiles?.length ?? 0);
        updatePathsList(allFiles);
        updateFileList(filesList);
    });
    document.getElementById('browse-list-btn')!.addEventListener('click', async () => {
        const file: string | null = await open({
            multiple: false,
            directory: false,
        });

        allFiles = [];
        filesList = file;

        updateFileCount(0);
        updatePathsList(allFiles);
        updateFileList(filesList);
    });

    document.getElementById('start-btn')!.addEventListener('click', () => {
        const threadCountInput: HTMLInputElement = document.getElementById('thread-count')! as HTMLInputElement;
        const ffmpegOptionsInput: HTMLTextAreaElement = document.getElementById('ffmpeg-options')! as HTMLTextAreaElement;
        const outputPatternInput: HTMLInputElement = document.getElementById('output-pattern')! as HTMLInputElement;
        const overwriteCheckbox: HTMLInputElement = document.getElementById('overwrite')! as HTMLInputElement;
        const verboseCheckbox: HTMLInputElement = document.getElementById('verbose')! as HTMLInputElement;
        const deleteCheckbox: HTMLInputElement = document.getElementById('delete-source')! as HTMLInputElement;

        const args: CmdArgs = {
            thread_count: Number(threadCountInput.value),
            ffmpeg_options: ffmpegOptionsInput.value ? ffmpegOptionsInput.value : null,
            input: allFiles ? allFiles : null,
            file_list: filesList ? filesList : null,
            overwrite: overwriteCheckbox.checked,
            verbose: verboseCheckbox.checked,
            delete: deleteCheckbox.checked,
            eta: false,
            output: outputPatternInput.value
        };

        doneFiles = 0;

        updateProgressBar(doneFiles, totalFiles);
        showProgressBar();
        showLogSection(verboseCheckbox.checked);
        clearLogSection();

        invoke('start_job', { options: JSON.stringify(args) });
    });

    listen<number>('update-total-file-count', event => totalFiles = event.payload);

    listen<number>('progress-update', event => {
        doneFiles = event.payload;
        updateProgressBar(doneFiles, totalFiles);
    });

    listen<string>('log-update-info', event => {
        updateLog(event.payload, LogSeverity.INFO);
    });

    listen<string>('log-update-error', event => {
        updateLog(event.payload, LogSeverity.ERROR);
    });
});

function updateLog(toWrite: string, severity: LogSeverity): void {
    const logEntry: HTMLSpanElement = document.createElement('span');
    logEntry.innerText = toWrite;
    logEntry.classList.add('log-entry', severity === LogSeverity.INFO ? 'info' : 'error');

    document.getElementById('log-content')?.appendChild(logEntry);
}

function updateFileCount(count: number): void {
    document.getElementById('file-amount')!.innerText = `${count}`;
}

function updatePathsList(files: string[] | null): void {
    const fileListElement: HTMLElement = document.getElementById('file-list')!;

    if (!files) {
        return;
    }

    fileListElement.innerText = files.length ? files.join('\n') : 'None selected';
}

function updateFileList(path: string | null): void {
    document.getElementById('file-list-path')!.innerText = path ? path : 'None selected';
}

function updateProgressBar(done: number, total: number): void {
    const progressBar = document.getElementById('progress-bar') as HTMLProgressElement;
    const progressText = document.getElementById('progress-text');
    if (progressBar && progressText) {
        progressBar.max = total;
        progressBar.value = done;
        progressText.textContent = `${done} / ${total} files processed`;
    }
}

function showProgressBar(): void {
    const progressSection = document.getElementById('progress-bar-section');
    if (progressSection) {
        progressSection.style.display = 'block';
    }
}

function showLogSection(show: boolean): void {
    if (!show) {
        return;
    }

    document.getElementById('log-section')!.style.display = 'block';
}

function clearLogSection(): void {
    document.getElementById('log-content')!.innerHTML = '';
}
