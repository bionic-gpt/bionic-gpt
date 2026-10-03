# Homepage photographs

These files are local derivatives of the requested Pexels photographs:

| Scene | Photographer | Source |
| --- | --- | --- |
| Office | Antoni Shkraba | https://www.pexels.com/photo/business-people-working-in-the-office-7163395/ |
| Paintball | José Alcalá | https://www.pexels.com/photo/paintball-player-in-action-during-outdoor-game-34383714/ |
| Evening | Imperio Ame | https://www.pexels.com/photo/elderly-man-sitting-in-a-garden-with-a-glass-of-red-wine-15063446/ |

Original downloads: `https://images.pexels.com/photos/{id}/pexels-photo-{id}.jpeg`.
Only optimized derivatives are shipped; the homepage makes no Pexels requests.

Conversion settings: Pillow 11.1, EXIF orientation applied, RGB, proportional
Lanczos resize to widths 768/1280/1920, metadata removed. WebP uses quality 78,
method 6. AVIF uses libavif 1.2.1 (`avifenc -q 48 -s 6 -j 2 -y 420
--ignore-exif --ignore-xmp`) from the resized RGB PNG, avoiding recompression
through JPEG. The original downloads and intermediate PNGs stay outside the site.

Display uses `object-fit: cover`; desktop and mobile focal positions live with
the scene data in `src/marketing/use_case_hero.rs`.
