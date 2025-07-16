import { open } from '@tauri-apps/plugin-dialog';

document.addEventListener('DOMContentLoaded', () => {
    let allFiles: string[] | null = null;
    let filesList: string | null = null;

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