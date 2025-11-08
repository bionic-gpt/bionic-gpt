/*! instant.page v5.2.0 - (C) 2019-2025 Alexandre Dieulot - https://instant.page/license */

/** ES module version with strict TS types + type guard narrowing */

let _allowQueryString = false;
let _allowExternalLinks = false;
let _useWhitelist = false;
let _delayOnHover = 65;
let _lastTouchstartEvent: TouchEvent | null = null;
let _mouseoverTimer: number | null = null;
const _preloadedList = new Set<string>();

export function initInstantPage(): void {
  const supportChecksRelList = document.createElement('link').relList;

  const supportsPrefetch = !!supportChecksRelList?.supports?.('prefetch');
  if (!supportsPrefetch) return;

  const useMousedownShortcut = 'instantMousedownShortcut' in document.body.dataset;
  _allowQueryString = 'instantAllowQueryString' in document.body.dataset;
  _allowExternalLinks = 'instantAllowExternalLinks' in document.body.dataset;
  _useWhitelist = 'instantWhitelist' in document.body.dataset;

  let preloadOnMousedown = false;
  let preloadOnlyOnMousedown = false;
  let preloadWhenVisible = false;

  if ('instantIntensity' in document.body.dataset) {
    const param = document.body.dataset.instantIntensity!;

    if (param === 'mousedown' && !useMousedownShortcut) preloadOnMousedown = true;
    if (param === 'mousedown-only' && !useMousedownShortcut) {
      preloadOnMousedown = true;
      preloadOnlyOnMousedown = true;
    }

    if (param === 'viewport' || param === 'viewport-all') {
      const isSmall = document.documentElement.clientWidth * document.documentElement.clientHeight < 450_000;
      const connection = (navigator as any).connection as { saveData?: boolean; effectiveType?: string } | undefined;
      const slow = !!(connection?.saveData || connection?.effectiveType?.includes?.('2g'));
      if (param === 'viewport-all' || (isSmall && !slow)) preloadWhenVisible = true;
    }

    const asInt = parseInt(param, 10);
    if (!Number.isNaN(asInt)) _delayOnHover = asInt;
  }

  const listenerOpts: AddEventListenerOptions = { capture: true, passive: true };

  document.addEventListener(
    'touchstart',
    preloadOnlyOnMousedown ? touchstartEmptyListener : touchstartListener,
    listenerOpts
  );

  if (!preloadOnMousedown) {
    document.addEventListener('mouseover', mouseoverListener, listenerOpts);
  } else {
    document.addEventListener('mousedown', mousedownListener, listenerOpts);
  }

  if (useMousedownShortcut) {
    document.addEventListener('mousedown', mousedownShortcutListener, listenerOpts);
  }

  if (preloadWhenVisible) {
    const idle: (cb: () => void, opts?: { timeout?: number }) => void =
      (window as any).requestIdleCallback || ((cb) => cb());
    idle(() => {
      const obs = new IntersectionObserver((entries) => {
        for (const e of entries) {
          if (e.isIntersecting) {
            obs.unobserve(e.target);
            preload((e.target as HTMLAnchorElement).href);
          }
        }
      });

      document.querySelectorAll<HTMLAnchorElement>('a').forEach((a) => {
        if (isPreloadable(a)) obs.observe(a);
      });
    }, { timeout: 1500 });
  }
}

/** --- Event handlers --- */

function touchstartListener(e: TouchEvent): void {
  _lastTouchstartEvent = e;
  const a = (e.target as Element | null)?.closest?.('a') as HTMLAnchorElement | null;
  if (!isPreloadable(a)) return;       // type guard narrows a
  preload(a.href, 'high');
}

function touchstartEmptyListener(e: TouchEvent): void {
  _lastTouchstartEvent = e;
}

function mouseoverListener(e: MouseEvent): void {
  if (isLikelyTouch(e)) return;
  const a = (e.target as Element | null)?.closest?.('a') as HTMLAnchorElement | null;
  if (!isPreloadable(a)) return;       // narrowed
  a.addEventListener('mouseout', mouseoutListener, { passive: true });
  _mouseoverTimer = window.setTimeout(() => {
    preload(a.href, 'high');
    _mouseoverTimer = null;
  }, _delayOnHover);
}

function mousedownListener(e: MouseEvent): void {
  if (isLikelyTouch(e)) return;
  const a = (e.target as Element | null)?.closest?.('a') as HTMLAnchorElement | null;
  if (!isPreloadable(a)) return;       // narrowed
  preload(a.href, 'high');
}

function mouseoutListener(e: MouseEvent): void {
  // if moving between the same <a>, ignore
  const fromA = (e.target as Element | null)?.closest?.('a');
  const toA = (e.relatedTarget as Element | null)?.closest?.('a');
  if (e.relatedTarget && fromA && fromA === toA) return;

  if (_mouseoverTimer) {
    clearTimeout(_mouseoverTimer);
    _mouseoverTimer = null;
  }
}

function mousedownShortcutListener(e: MouseEvent): void {
  if (isLikelyTouch(e)) return;
  const a = (e.target as Element | null)?.closest?.('a') as HTMLAnchorElement | null;
  if (!a || e.which > 1 || e.metaKey || e.ctrlKey) return;

  a.addEventListener(
    'click',
    (ev) => {
      if ((ev as any).detail != 1337) ev.preventDefault();
    },
    { capture: true, passive: false, once: true }
  );

  a.dispatchEvent(new MouseEvent('click', { view: window, bubbles: true, cancelable: false, detail: 1337 }));
}

/** --- Helpers --- */

function isLikelyTouch(e: MouseEvent | Event): boolean {
  if (!_lastTouchstartEvent) return false;
  if (e.target !== _lastTouchstartEvent.target) return false;
  return (e as any).timeStamp - _lastTouchstartEvent.timeStamp < 2500;
}

/** Type guard: narrows to HTMLAnchorElement when preloadable */
function isPreloadable(a: HTMLAnchorElement | null | undefined): a is HTMLAnchorElement {
  if (!a || !a.href) return false;

  if (_useWhitelist && !('instant' in a.dataset)) return false;

  if (a.origin !== location.origin) {
    const allowed = _allowExternalLinks || 'instant' in a.dataset;
    if (!allowed) return false;
  }

  if (!['http:', 'https:'].includes(a.protocol)) return false;
  if (a.protocol === 'http:' && location.protocol === 'https:') return false;

  if (!_allowQueryString && a.search && !('instant' in a.dataset)) return false;

  if (a.hash && a.pathname + a.search === location.pathname + location.search) return false;

  if ('noInstant' in a.dataset) return false;

  return true;
}

function preload(url: string, priority: 'auto' | 'high' = 'auto'): void {
  if (_preloadedList.has(url)) return;

  const l = document.createElement('link');
  l.rel = 'prefetch';
  l.href = url;
  (l as any).fetchPriority = priority; // not in all lib.dom.d.ts yet
  l.as = 'document';                   // enables “restrictive prefetch” behavior in Chromium
  document.head.appendChild(l);

  _preloadedList.add(url);
}

/** Example usage:
 * import { initInstantPage } from './instant-page-module';
 * initInstantPage();
 */
