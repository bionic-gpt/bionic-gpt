/**
 * File upload functionality for the console
 * - Shows upload progress
 * - Allows multiple file uploads
 * - Allows removing files before submission
 * - Files are only sent when the user submits the prompt
 */
export function fileUpload() {
  // Find the attach button
  const attachButton = document.getElementById('attach-button');
  if (!attachButton) return;

  // Create a hidden file input element
  const fileInput = document.createElement('input');
  fileInput.type = 'file';
  fileInput.multiple = true;
  fileInput.style.display = 'none';
  fileInput.id = 'file-upload-input';
  document.body.appendChild(fileInput);

  // Create a container for the file list
  const fileListContainer = document.createElement('div');
  fileListContainer.className = 'mt-2 space-y-2 file-list-container';
  fileListContainer.style.display = 'none';
  
  // Find the form and insert the file list container after the textarea
  const form = attachButton.closest('form');
  const textarea = form?.querySelector('textarea');
  if (form && textarea) {
    textarea.parentNode?.insertBefore(fileListContainer, textarea.nextSibling);
  } else {
    return; // Exit if we can't find the form or textarea
  }

  // Store selected files
  const selectedFiles = new Map<string, File>();

  // Attach button click handler
  attachButton.addEventListener('click', (e) => {
    e.preventDefault();
    fileInput.click();
  });

  // File selection handler
  fileInput.addEventListener('change', () => {
    if (fileInput.files && fileInput.files.length > 0) {
      // Add each file to our collection
      Array.from(fileInput.files).forEach(file => {
        // Use a unique key combining name and last modified time
        const key = `${file.name}-${file.lastModified}`;
        if (!selectedFiles.has(key)) {
          selectedFiles.set(key, file);
          addFileToList(key, file);
        }
      });
      
      // Show the file list container if we have files
      if (selectedFiles.size > 0) {
        fileListContainer.style.display = 'block';
      }
      
      // Reset the file input
      fileInput.value = '';
    }
  });

  // Add a file to the list
  function addFileToList(key: string, file: File) {
    // Create file item container
    const fileItem = document.createElement('div');
    fileItem.className = 'flex items-center justify-between p-2 border rounded';
    fileItem.dataset.key = key;
    
    // File info
    const fileInfo = document.createElement('div');
    fileInfo.className = 'flex-1';
    
    const fileName = document.createElement('span');
    fileName.className = 'font-medium';
    fileName.textContent = file.name;
    
    const fileSize = document.createElement('span');
    fileSize.className = 'text-xs text-gray-500 ml-2';
    fileSize.textContent = `(${formatFileSize(file.size)})`;
    
    fileInfo.appendChild(fileName);
    fileInfo.appendChild(fileSize);
    
    // Remove button
    const removeButton = document.createElement('button');
    removeButton.className = 'text-red-500 ml-2';
    removeButton.textContent = 'Remove';
    removeButton.addEventListener('click', () => removeFile(key));
    
    // Progress bar container
    const progressContainer = document.createElement('div');
    progressContainer.className = 'w-full h-1 bg-gray-200 rounded mt-1';
    progressContainer.style.display = 'none';
    
    // Progress bar
    const progressBar = document.createElement('div');
    progressBar.className = 'h-1 bg-blue-500 rounded';
    progressBar.style.width = '0%';
    
    progressContainer.appendChild(progressBar);
    
    // Assemble the file item
    fileItem.appendChild(fileInfo);
    fileItem.appendChild(removeButton);
    fileItem.appendChild(progressContainer);
    
    // Add to the container
    fileListContainer.appendChild(fileItem);
  }

  // Remove a file from the list
  function removeFile(key: string) {
    selectedFiles.delete(key);
    const fileItem = fileListContainer.querySelector(`[data-key="${key}"]`);
    if (fileItem) {
      fileListContainer.removeChild(fileItem);
    }
    
    // Hide the container if no files are left
    if (selectedFiles.size === 0) {
      fileListContainer.style.display = 'none';
    }
  }

  // Format file size for display
  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  }

  // Handle form submission
  form.addEventListener('submit', async (e) => {
    // Only intercept if we have files to upload
    if (selectedFiles.size > 0) {
      e.preventDefault();
      
      console.log('Form submission with files detected');
      
      // Create FormData from the form
      const formData = new FormData(form);
      
      // Log the form data before adding files
      console.log('Form data before adding files:', Array.from(formData.entries()));
      
      // Add each file to the FormData
      selectedFiles.forEach((file, key) => {
        console.log(`Adding file to FormData: ${file.name} (${formatFileSize(file.size)})`);
        formData.append('attachments', file);
      });
      
      try {
        
        console.log('Submitting form with files');
        
        // Submit the form with files using fetch API
        const response = await fetch(form.action, {
          method: 'POST',
          body: formData,
          // No need to set Content-Type header, it will be set automatically with boundary
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        // Handle the response - redirect to the URL returned by the server
        const redirectUrl = response.url;
        window.location.href = redirectUrl;
      } catch (error) {
        console.error('Upload failed:', error);
        alert('File upload failed. Please try again.');
      }
    }
  });
}