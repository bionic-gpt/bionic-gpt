export const initializeConsoleForm = () => {
  new DynamicForm();
};

class DynamicForm {
  private form: HTMLFormElement;
  private textarea: HTMLTextAreaElement;
  private fileInput: HTMLInputElement;
  private fileList: HTMLDivElement;
  private files: File[] = [];

  constructor() {
    this.fileInput = document.getElementById('fileInput') as HTMLInputElement;
    this.fileList = document.getElementById('fileList') as HTMLDivElement;

    if (this.fileInput && this.fileList) {
      this.initializeEventListeners();
    }
  }

  private initializeEventListeners(): void {
    this.fileInput.addEventListener('change', this.handleFileChange.bind(this));
    this.fileList.addEventListener('click', this.handleFileDelete.bind(this));
  }

  private handleFileChange(e: Event): void {
    const target = e.target as HTMLInputElement;
    if (target.files) {
      const newFiles = Array.from(target.files);
      this.files = [...this.files, ...newFiles];
      this.updateFileList();
    }
  }

  private updateFileList(): void {
    this.fileList.innerHTML = '';
    this.files.forEach((file, index) => {
      const fileElement = document.createElement('div');
      fileElement.className = 'badge badge-primary gap-2';
      fileElement.innerHTML = `
        ${file.name}
        <button type="button" class="btn btn-ghost btn-xs p-0" data-index="${index}">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      `;
      this.fileList.appendChild(fileElement);
    });
  }

  private handleFileDelete(e: MouseEvent): void {
    const target = e.target as HTMLElement;
    if (target.closest('button')) {
      const index = parseInt(target.closest('button')!.getAttribute('data-index')!);
      this.files.splice(index, 1);
      this.updateFileList();
    }
  }
}
