import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { CmdArgs, LogSeverity } from './models';
import { listen } from '@tauri-apps/api/event';
import {
  updateFileCount,
  updatePathsList,
  updateFileList,
  updateProgressBar,
  showProgressBar,
  showLogSection,
  clearLogSection,
  addSpacerToLog,
  updateLog,
  prepareTabs,
  validateButton,
  showStopButton,
  hideStopButton,
} from './dom';

document.addEventListener('DOMContentLoaded', () => {
  let allFiles: string[] | null = null;
  let filesList: string | null = null;
  let totalFiles: number = 0;
  let doneFiles: number = 0;

  prepareTabs();

  (document.getElementById('start-btn') as HTMLButtonElement).disabled = true;

  const browseFilesBtn = document.getElementById(
    'browse-files-btn'
  ) as HTMLButtonElement;
  const browseListBtn = document.getElementById(
    'browse-list-btn'
  ) as HTMLButtonElement;

  browseFilesBtn.addEventListener('click', async () => {
    const files: string[] | null = await open({
      multiple: true,
      directory: false,
    });

    allFiles = files;
    filesList = null;

    updateFileCount(allFiles?.length ?? 0);
    updatePathsList(allFiles);
    updateFileList(filesList);
    validateButton(allFiles, filesList);
  });
  browseListBtn.addEventListener('click', async () => {
    const file: string | null = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'File Lists',
          extensions: ['txt', 'lst', 'list'],
        },
      ],
    });

    allFiles = [];
    filesList = file;

    updateFileCount(0);
    updatePathsList(allFiles);
    updateFileList(filesList);
    validateButton(allFiles, filesList);
  });

  document.getElementById('output-pattern')!.addEventListener('keyup', () => {
    validateButton(allFiles, filesList);
  });

  document.getElementById('start-btn')!.addEventListener('click', event => {
    const startBtn: HTMLButtonElement = event.target as HTMLButtonElement;
    const threadCountInput: HTMLInputElement = document.getElementById(
      'thread-count'
    )! as HTMLInputElement;
    const ffmpegOptionsInput: HTMLTextAreaElement = document.getElementById(
      'ffmpeg-options'
    )! as HTMLTextAreaElement;
    const outputPatternInput: HTMLInputElement = document.getElementById(
      'output-pattern'
    )! as HTMLInputElement;
    const overwriteCheckbox: HTMLInputElement = document.getElementById(
      'overwrite'
    )! as HTMLInputElement;
    const verboseCheckbox: HTMLInputElement = document.getElementById(
      'verbose'
    )! as HTMLInputElement;
    const deleteCheckbox: HTMLInputElement = document.getElementById(
      'delete-source'
    )! as HTMLInputElement;

    const args: CmdArgs = {
      thread_count: Number(threadCountInput.value),
      ffmpeg_options: ffmpegOptionsInput.value
        ? ffmpegOptionsInput.value
        : null,
      input: allFiles ? allFiles : null,
      file_list: filesList ? filesList : null,
      overwrite: overwriteCheckbox.checked,
      verbose: verboseCheckbox.checked,
      delete: deleteCheckbox.checked,
      eta: false,
      output: outputPatternInput.value,
    };

    doneFiles = 0;

    updateProgressBar(doneFiles, totalFiles);

    showProgressBar();

    showLogSection();
    clearLogSection();

    invoke('start_job', { options: JSON.stringify(args) });
    updateLog('Job has started, please wait...', LogSeverity.INFO);
    showStopButton();

    startBtn.disabled = true;
    browseFilesBtn.disabled = true;
    browseListBtn.disabled = true;
    overwriteCheckbox.disabled = true;
    verboseCheckbox.disabled = true;
    deleteCheckbox.disabled = true;
  });

  document.getElementById('stop-btn')!.addEventListener('click', () => {
    invoke('stop_job');
    updateLog('Stopping job(s)...', LogSeverity.ERROR);
  });

  listen<[string, number, string[]]>('job-finished', event => {
    const startBtn: HTMLButtonElement = document.getElementById(
      'start-btn'
    ) as HTMLButtonElement;
    const overWriteCheckBox: HTMLInputElement = document.getElementById(
      'overwrite'
    ) as HTMLInputElement;
    const verboseCheckBox: HTMLInputElement = document.getElementById(
      'verbose'
    ) as HTMLInputElement;
    const deleteSourceCheckBox: HTMLInputElement = document.getElementById(
      'delete-source'
    ) as HTMLInputElement;

    startBtn.disabled = false;
    browseFilesBtn.disabled = false;
    browseListBtn.disabled = false;
    overWriteCheckBox.disabled = false;
    verboseCheckBox.disabled = false;
    deleteSourceCheckBox.disabled = false;

    hideStopButton();

    addSpacerToLog();

    const successfulFiles: number = event.payload[1];
    const logLine: string = `${successfulFiles} out of ${totalFiles} files have been successful. A detailed log has been written to ${event.payload[0]}`;
    updateLog(
      logLine,
      totalFiles !== successfulFiles ? LogSeverity.ERROR : LogSeverity.INFO
    );

    const failedPaths: string[] = event.payload[2];
    if (failedPaths.length > 0 && verboseCheckBox.checked) {
      addSpacerToLog();

      const staticLine: string =
        'The following files were not processed due to the errors above:';
      const failedPathsList: string = failedPaths.join('\n');
      const finalLine: string = `${staticLine}\n${failedPathsList}`;

      updateLog(finalLine, LogSeverity.ERROR);
    }
  });

  listen<string>('general-ffmpeg-error', event => {
    updateLog(event.payload, LogSeverity.ERROR);
  });

  listen<number>(
    'update-total-file-count',
    event => (totalFiles = event.payload)
  );

  listen<number>('progress-update', event => {
    doneFiles = event.payload;
    updateProgressBar(doneFiles, totalFiles);
  });

  listen<string>('log-update-info', event => {
    console.log(event.payload);
    updateLog(event.payload, LogSeverity.INFO);
  });

  listen<string>('log-update-error', event => {
    updateLog(event.payload, LogSeverity.ERROR);
  });
});
