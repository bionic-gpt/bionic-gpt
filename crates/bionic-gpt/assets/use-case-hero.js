const TASK_DURATION = 5000;
const FADE_DURATION = 800;
const LOAD_TIMEOUT = 12000;

for (const hero of document.querySelectorAll('[data-use-case-hero]')) {
  const pictures = [...hero.querySelectorAll('picture[data-scene]')];
  const scenes = pictures.map((picture) => ({
    picture,
    image: picture.querySelector('img'),
    top: picture.dataset.top,
    bottom: picture.dataset.bottom,
    tasks: [...hero.querySelectorAll('.use-case-hero__task')].filter(
      (task) => task.dataset.scene === picture.dataset.scene,
    ),
  }));
  const control = hero.querySelector('.use-case-hero__pause');
  const topLine = hero.querySelector('.use-case-hero__top');
  const bottomLine = hero.querySelector('.use-case-hero__bottom');
  const motion = matchMedia('(prefers-reduced-motion: reduce)');
  const loads = new Map();
  let sceneIndex = 0;
  let taskIndex = 0;
  let paused = false;
  let timer;
  let fadeTimer;
  let finishFade;
  let generation = 0;

  const running = () => !paused && !motion.matches && !document.hidden;

  function showTask() {
    const current = scenes[sceneIndex];
    // Static copy changes only at scene boundaries, never during task rotation.
    if (topLine.textContent !== current.top) topLine.textContent = current.top;
    if (bottomLine.textContent !== current.bottom) bottomLine.textContent = current.bottom;
    for (const [index, scene] of scenes.entries()) {
      for (const [task, element] of scene.tasks.entries()) {
        const active = index === sceneIndex && task === taskIndex;
        const leaving = element.classList.contains('is-active') && !active;
        element.classList.toggle('is-leaving', leaving);
        element.classList.toggle('is-active', active);
        element.setAttribute('aria-hidden', String(!active));
      }
    }
  }

  function loadScene(index) {
    if (loads.has(index)) return loads.get(index);
    const { picture, image } = scenes[index];
    const load = new Promise((resolve) => {
      let settled = false;
      const timeout = setTimeout(() => finish(false), LOAD_TIMEOUT);
      function finish(ready) {
        if (settled) return;
        settled = true;
        clearTimeout(timeout);
        resolve(ready);
      }
      // Sources must be activated before the fallback so the browser chooses
      // just one supported format and size. No scene URLs are hotlinked.
      for (const source of picture.querySelectorAll('source[data-srcset]')) {
        source.srcset = source.dataset.srcset;
        delete source.dataset.srcset;
      }
      if (image.dataset.srcset) {
        image.srcset = image.dataset.srcset;
        image.src = image.dataset.src;
        delete image.dataset.srcset;
        delete image.dataset.src;
      }
      image.decode().then(
        () => finish(image.naturalWidth > 0),
        () => finish(false),
      );
    });
    loads.set(index, load);
    return load;
  }

  async function nextScene() {
    for (let offset = 1; offset < scenes.length; offset++) {
      const index = (sceneIndex + offset) % scenes.length;
      if (await loadScene(index)) return index;
      // Respect pause, visibility and motion changes while awaiting decoding.
      if (!running()) return sceneIndex;
    }
    return sceneIndex;
  }

  function crossfade(index) {
    finishFade?.();
    const outgoing = scenes[sceneIndex].picture;
    const incoming = scenes[index].picture;
    // Leave the outgoing photo opaque beneath the incoming photo, avoiding
    // a dip to the background halfway through the crossfade (including wrap).
    outgoing.style.zIndex = '0';
    incoming.classList.add('is-active');
    finishFade = () => {
      clearTimeout(fadeTimer);
      outgoing.classList.remove('is-active');
      outgoing.style.zIndex = '';
      finishFade = undefined;
    };
    fadeTimer = setTimeout(() => finishFade?.(), FADE_DURATION);
    sceneIndex = index;
  }

  function schedule() {
    clearTimeout(timer);
    if (running()) timer = setTimeout(advance, TASK_DURATION);
  }

  async function advance() {
    const currentGeneration = generation;
    if (!running()) return;
    if (taskIndex < scenes[sceneIndex].tasks.length - 1) {
      taskIndex++;
      // Wait until the first task has been read before fetching another photo.
      // A slow initial image must finish before competing requests are started.
      if (taskIndex === 1 && scenes[sceneIndex].image.complete) {
        void loadScene((sceneIndex + 1) % scenes.length);
      }
    } else {
      const index = await nextScene();
      if (!running() || generation !== currentGeneration) return;
      if (index !== sceneIndex) crossfade(index);
      taskIndex = 0;
    }
    showTask();
    schedule();
  }

  function sync() {
    generation++;
    clearTimeout(timer);
    control.hidden = motion.matches;
    control.textContent = paused ? 'Resume' : 'Pause';
    control.setAttribute('aria-label', paused ? 'Resume hero rotation' : 'Pause hero rotation');
    control.setAttribute('aria-pressed', String(paused));
    if (motion.matches) {
      finishFade?.();
      sceneIndex = 0;
      taskIndex = 0;
      for (const [index, scene] of scenes.entries()) {
        scene.picture.classList.toggle('is-active', index === 0);
      }
      showTask();
    }
    schedule();
  }

  control.addEventListener('click', () => {
    paused = !paused;
    sync();
  });
  motion.addEventListener('change', sync);
  document.addEventListener('visibilitychange', sync);
  sync();
}
