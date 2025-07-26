import { LogSeverity } from './models';

export function addSpacerToLog(): void {
  const logContent: HTMLElement = document.getElementById('log-content')!;
  const spacer: HTMLParagraphElement = document.createElement('p');
  spacer.innerHTML = '&nbsp;';

  logContent.appendChild(spacer);
}

export function updateLog(toWrite: string, severity: LogSeverity): void {
  const logEntry: HTMLDivElement = document.createElement('div');
  logEntry.innerText = toWrite;
  logEntry.classList.add(
    'log-entry',
    severity === LogSeverity.INFO ? 'info' : 'error'
  );

  document.getElementById('log-content')?.appendChild(logEntry);
}

export function updateFileCount(count: number): void {
  document.getElementById('file-amount')!.innerText = `${count}`;
}

export function updatePathsList(files: string[] | null): void {
  const fileListElement: HTMLElement = document.getElementById('file-list')!;

  if (!files) {
    return;
  }

  fileListElement.innerText = files.length ? files.join('\n') : 'None selected';
}

export function updateFileList(path: string | null): void {
  document.getElementById('file-list-path')!.innerText = path
    ? path
    : 'None selected';
}

export function updateProgressBar(done: number, total: number): void {
  const progressBar = document.getElementById(
    'progress-bar'
  ) as HTMLProgressElement;
  const progressText = document.getElementById('progress-text');
  if (progressBar && progressText) {
    progressBar.max = total;
    progressBar.value = done;
    progressText.textContent = `${done} / ${total} files processed`;
  }
}

export function showProgressBar(): void {
  const progressSection = document.getElementById('progress-bar-section');
  if (progressSection) {
    progressSection.style.display = 'block';
  }
}

export function showLogSection(): void {
  document.getElementById('log-section')!.style.display = 'block';
}

export function clearLogSection(): void {
  document.getElementById('log-content')!.innerHTML = '';
}

export function prepareTabs(): void {
  const tabButtons: NodeListOf<Element> = document.querySelectorAll('.tab-btn');
  const tabContents: NodeListOf<Element> =
    document.querySelectorAll('.tab-content');

  tabButtons.forEach((button: Element) => {
    button.addEventListener('click', () => {
      const targetTab: string | null = button.getAttribute('data-tab');

      tabButtons.forEach((btn: Element) => btn.classList.remove('active'));
      tabContents.forEach((content: Element) =>
        content.classList.remove('active')
      );

      button.classList.add('active');
      const targetContent: HTMLElement | null = document.getElementById(
        `${targetTab}-tab`
      );
      if (targetContent) {
        targetContent.classList.add('active');
      }
    });
  });
}

export function validateButton(
  allFiles: string[] | null,
  fileList: string | null
): void {
  const button: HTMLButtonElement = document.getElementById(
    'start-btn'
  ) as HTMLButtonElement;
  const outputPattern: HTMLInputElement = document.getElementById(
    'output-pattern'
  ) as HTMLInputElement;

  const pattern: string = outputPattern.value.trim();
  const totalFiles: number = allFiles?.length ?? 0;

  if (pattern !== '' && (totalFiles > 0 || !!fileList)) {
    button.disabled = false;
  } else {
    button.disabled = true;
  }
}

export function showStopButton(): void {
  const stopBtn: HTMLButtonElement = document.getElementById(
    'stop-btn'
  ) as HTMLButtonElement;
  stopBtn.style.display = 'inline-block';
}

export function hideStopButton(): void {
  const stopBtn: HTMLButtonElement = document.getElementById(
    'stop-btn'
  ) as HTMLButtonElement;
  stopBtn.style.display = 'none';
}
