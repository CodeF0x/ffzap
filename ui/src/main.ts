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
} from './dom';

document.addEventListener('DOMContentLoaded', () => {
  let allFiles: string[] | null = null;
  let filesList: string | null = null;
  let totalFiles: number = 0;
  let doneFiles: number = 0;

  prepareTabs();

  document
    .getElementById('browse-files-btn')!
    .addEventListener('click', async () => {
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
  document
    .getElementById('browse-list-btn')!
    .addEventListener('click', async () => {
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
    showLogSection(verboseCheckbox.checked);
    clearLogSection();

    invoke('start_job', { options: JSON.stringify(args) });
    startBtn.disabled = true;
  });

  listen<[string, number, string[]]>('job-finished', event => {
    (document.getElementById('start-btn') as HTMLButtonElement).disabled =
      false;

    addSpacerToLog();

    const successfulFiles: number = event.payload[1];
    const logLine: string = `${successfulFiles} out of ${totalFiles} files have been successful. A detailed log has been written to ${event.payload[0]}`;
    updateLog(
      logLine,
      totalFiles !== successfulFiles ? LogSeverity.ERROR : LogSeverity.INFO
    );

    const failedPaths: string[] = event.payload[2];
    if (failedPaths.length > 0) {
      addSpacerToLog();

      const staticLine: string =
        'The following files were not processed due to the errors above:';
      const failedPathsList: string = failedPaths.join('\n');
      const finalLine: string = `${staticLine}\n${failedPathsList}`;

      updateLog(finalLine, LogSeverity.ERROR);
    }
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
    updateLog(event.payload, LogSeverity.INFO);
  });

  listen<string>('log-update-error', event => {
    updateLog(event.payload, LogSeverity.ERROR);
  });

  listen('job-finished', () => {
    const startBtn: HTMLButtonElement = document.getElementById(
      'start-btn'
    )! as HTMLButtonElement;
    startBtn.disabled = false;
  });
});
