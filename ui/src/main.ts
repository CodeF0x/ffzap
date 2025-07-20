import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { CmdArgs } from './models';
import { listen } from '@tauri-apps/api/event';

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
        const threadCountInput = document.getElementById('thread-count') as HTMLInputElement;
        const ffmpegOptionsInput = document.getElementById('ffmpeg-options') as HTMLTextAreaElement;
        const outputPatternInput = document.getElementById('output-pattern') as HTMLInputElement;
        const overwriteCheckbox = document.getElementById('overwrite') as HTMLInputElement;
        const verboseCheckbox = document.getElementById('verbose') as HTMLInputElement;
        const deleteCheckbox = document.getElementById('delete-source') as HTMLInputElement;

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

        if (allFiles && allFiles.length > 0) {
            totalFiles = allFiles.length;
        } else if (filesList) {
            // If using a file list, you may want to fetch the count from the backend if not known
            // For now, set to 0 (or update this logic as needed)
            totalFiles = 0;
        } else {
            totalFiles = 0;
        }
        doneFiles = 0;
        updateProgressBar(doneFiles, totalFiles);
        showProgressBar();
        invoke('start_job', { options: JSON.stringify(args) });
    });

    // Listen for progress-update events from backend
    listen<number>('progress-update', (event) => {
        console.log(event);
        doneFiles = event.payload;
        updateProgressBar(doneFiles, totalFiles);
    });
});

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