export const examplePrompts = () => {
  const buttons = document.querySelectorAll<HTMLButtonElement>('.example-prompt');
  const textarea = document.querySelector<HTMLTextAreaElement>('textarea[name="message"]');
  buttons.forEach(button => {
    button.addEventListener('click', (e) => {
      e.preventDefault();
      const example = button.dataset.example;
      if (example && textarea) {
        textarea.value = example;
        const event = new Event('input', { bubbles: true });
        textarea.dispatchEvent(event);
      }
    });
  });
};
