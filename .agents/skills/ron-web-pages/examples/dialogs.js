// Local enhancement for the data attributes rendered by DeleteConfirmation.
document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const opener = event.target.closest("[data-open-dialog]");
  if (opener) {
    const dialog = document.getElementById(opener.getAttribute("data-open-dialog"));
    if (dialog instanceof HTMLDialogElement) dialog.showModal();
  }
  const closer = event.target.closest("[data-close-dialog]");
  if (closer) {
    const dialog = closer.closest("dialog");
    if (dialog instanceof HTMLDialogElement) dialog.close();
  }
});
