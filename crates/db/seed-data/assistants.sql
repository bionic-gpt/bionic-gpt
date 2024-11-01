-- First, delete any existing prompts with type 'Assistant'
DELETE FROM prompts WHERE prompt_type = 'Assistant';

-- Insert a new object record into the 'objects' table
INSERT INTO objects (team_id, object_name, object_data, mime_type, file_name, file_size, file_hash, created_by, created_at, updated_at)
VALUES (
  (SELECT id FROM teams ORDER BY id LIMIT 1),
  'example_object_name',
  '<svg viewBox="0 0 1024 1024" class="icon" version="1.1" xmlns="http://www.w3.org/2000/svg" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"><path d="M96.4 364.2h830.3v529.5H96.4z" fill="#FFFFFF"></path><path d="M926.7 901.7h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-22.3-9.7h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 860h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 828h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 796h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 764h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 732h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 700h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 668h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 636h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 604h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 572h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 540h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 508h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 476h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 444h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 412h-16v-16h16v16z m830.3-13.9h-16v-16h16v16zM104.4 380h-16v-23.8h8.2v8h7.8V380z m822.3-7.8h-14.1v-16h22.1v9.9h-8v6.1z m-30.1 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z m-32 0h-16v-16h16v16z" fill="#0A0408"></path><path d="M172.6 438.3h677.8v381.4H172.6z" fill="#F4BE6F"></path><path d="M96.4 258.8h830.3v105.9H96.4z" fill="#55B7A8"></path><path d="M934.7 372.7H88.4V250.8h846.3v121.9z m-830.3-16h814.3v-89.9H104.4v89.9z" fill="#0A0408"></path><path d="M457.3 107.2h108.5v203.7H457.3z" fill="#EBB866"></path><path d="M573.8 318.9H449.3V99.2h124.5v219.7z m-108.5-16h92.5V115.2h-92.5v187.7z" fill="#0A0408"></path><path d="M308.7 560.2m-65.9 0a65.9 65.9 0 1 0 131.8 0 65.9 65.9 0 1 0-131.8 0Z" fill="#FFFFFF"></path><path d="M308.7 634.1c-40.8 0-73.9-33.2-73.9-73.9s33.2-73.9 73.9-73.9 73.9 33.2 73.9 73.9-33.1 73.9-73.9 73.9z m0-131.9c-32 0-57.9 26-57.9 57.9s26 57.9 57.9 57.9 57.9-26 57.9-57.9-25.9-57.9-57.9-57.9z" fill="#0A0408"></path><path d="M418.7 767.9c0-60.7-49.2-109.9-109.9-109.9s-109.9 49.2-109.9 109.9h219.8z" fill="#FFFFFF"></path><path d="M426.7 775.9H190.8v-8c0-65 52.9-117.9 117.9-117.9s117.9 52.9 117.9 117.9v8z m-219.6-16h203.3c-4.1-52.5-48.1-93.9-101.6-93.9s-97.6 41.4-101.7 93.9zM457.3 662.8h313.9v16H457.3zM457.3 751.2h261.8v16H457.3z" fill="#0A0408"></path><path d="M457.3 512.7h313.9v65.9H457.3z" fill="#FFFFFF"></path><path d="M779.2 586.7H449.3v-81.9h329.9v81.9z m-313.9-16h297.9v-49.9H465.3v49.9z" fill="#0A0408"></path><path d="M512 258.6m-20.3 0a20.3 20.3 0 1 0 40.6 0 20.3 20.3 0 1 0-40.6 0Z" fill="#FFFFFF"></path><path d="M512 287c-15.6 0-28.3-12.7-28.3-28.3s12.7-28.3 28.3-28.3 28.3 12.7 28.3 28.3S527.6 287 512 287z m0-40.7c-6.8 0-12.3 5.5-12.3 12.3S505.2 271 512 271s12.3-5.5 12.3-12.3-5.5-12.4-12.3-12.4z" fill="#0A0408"></path><path d="M71.5 868.8h49.8v49.8H71.5z" fill="#DC504F"></path><path d="M129.2 926.6H63.5v-65.8h65.8v65.8z m-49.7-16h33.8v-33.8H79.5v33.8z" fill="#0A0408"></path><path d="M899.4 868.8h49.8v49.8h-49.8z" fill="#DC504F"></path><path d="M957.1 926.6h-65.8v-65.8h65.8v65.8z m-49.7-16h33.8v-33.8h-33.8v33.8z" fill="#0A0408"></path></g></svg>',
  'image/svg+xml',
  'image0.svg',
  1024,
  'hash_of_the_file',
  (SELECT id FROM users ORDER BY id LIMIT 1),
  NOW(),
  NOW()
);

INSERT INTO objects (team_id, object_name, object_data, mime_type, file_name, file_size, file_hash, created_by, created_at, updated_at)
VALUES (
  (SELECT id FROM teams ORDER BY id LIMIT 1),
  'example_object_name',
  '<svg viewBox="0 0 1024 1024" class="icon" version="1.1" xmlns="http://www.w3.org/2000/svg" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"><path d="M511.9 129.1s357.7 40.8 441.4 441.4H730s-11.2-230.7-218.1-218.1V129.1z" fill="#FFFFFF"></path><path d="M953.9 571H729.5v-0.5c0-0.3-1.4-27.8-11.6-62.9-9.3-32.3-28.2-77.7-64.9-110.6-36.5-32.7-84-47.6-141-44.1h-0.5V128.5l0.6 0.1c0.9 0.1 90.7 10.9 189.5 70.6 58 35 107.8 79.8 147.9 133.1 50.1 66.6 85.3 146.7 104.4 238.1v0.6z m-223.5-1h222.2c-19.1-91-54.1-170.8-104.1-237.1-40-53.2-89.7-97.9-147.7-132.8-94.6-57.1-180.8-69.3-188.5-70.4v222.2c57.1-3.4 104.6 11.6 141.2 44.4 36.9 33.1 55.9 78.7 65.2 111.1 9.6 32.6 11.5 58.8 11.7 62.6z" fill=""></path><path d="M511.9 191.3c-209.4 0-379.1 169.7-379.1 379.1s169.7 379.1 379.1 379.1c209.4 0 379.1-169.7 379.1-379.1S721.3 191.3 511.9 191.3zM402.5 759.2c-10.7-6.2-20.9-13.3-30.3-21.2m0-0.1c-47.9-40-78.4-100.2-78.4-167.5 0-120.4 97.6-218.1 218.1-218.1C632.3 352.4 730 450 730 570.5s-97.6 218.1-218.1 218.1c-39.9 0-77.2-10.7-109.4-29.4" fill="#55B7A8"></path><path d="M511.9 957.6c-52.3 0-103-10.2-150.7-30.4-46.1-19.5-87.5-47.4-123.1-83-35.6-35.6-63.5-77-83-123.1-20.2-47.7-30.4-98.4-30.4-150.7s10.2-103 30.4-150.7c19.5-46.1 47.4-87.5 83-123.1 35.6-35.6 77-63.5 123.1-83 47.7-20.2 98.4-30.4 150.7-30.4s103 10.2 150.7 30.4c46.1 19.5 87.5 47.4 123.1 83 35.6 35.6 63.5 77 83 123.1 20.2 47.7 30.4 98.4 30.4 150.7s-10.2 103-30.4 150.7c-19.5 46.1-47.4 87.5-83 123.1-35.6 35.6-77 63.5-123.1 83-47.7 20.2-98.4 30.4-150.7 30.4z m0-758.3c-50.1 0-98.7 9.8-144.5 29.2-44.2 18.7-83.9 45.5-118 79.5-34.1 34.1-60.8 73.8-79.5 118-19.4 45.8-29.2 94.4-29.2 144.5s9.8 98.7 29.2 144.5c18.7 44.2 45.5 83.9 79.5 118 34.1 34.1 73.8 60.8 118 79.5 45.8 19.4 94.4 29.2 144.5 29.2 50.1 0 98.7-9.8 144.5-29.2 44.2-18.7 83.9-45.5 118-79.5 34.1-34.1 60.8-73.8 79.5-118 19.4-45.8 29.2-94.4 29.2-144.5s-9.8-98.7-29.2-144.5c-18.7-44.2-45.5-83.9-79.5-118-34.1-34.1-73.8-60.8-118-79.5-45.8-19.4-94.4-29.2-144.5-29.2z m0 597.2c-39.9 0-79.1-10.5-113.4-30.5-11-6.4-21.6-13.8-31.4-22-51.6-43.1-81.3-106.4-81.3-173.6 0-60.4 23.5-117.2 66.2-159.9s99.5-66.2 159.9-66.2 117.2 23.5 159.9 66.2S738 510 738 570.4c0 60.4-23.5 117.2-66.2 159.9s-99.5 66.2-159.9 66.2z m-105.4-44.2c31.9 18.5 68.3 28.3 105.4 28.3 115.8 0 210.1-94.2 210.1-210.1s-94.2-210.1-210.1-210.1c-115.8 0-210.1 94.2-210.1 210.1 0 62.5 27.5 121.3 75.5 161.3L373 737l4.3-5.2c9.2 7.6 19 14.5 29.2 20.5l-5 8.6 5-8.6z" fill="#0A0408"></path><path d="M511.9 352.4V62.8c-126.8 0-242.7 46.5-331.6 123.3l189.2 219.3c38.2-33 87.9-53 142.4-53z" fill="#FFFFFF"></path><path d="M368.6 416.6L169 185.3l6.1-5.2c45.8-39.6 97.7-70.5 154-91.9C387.4 66.1 449 54.8 511.9 54.8h8v305.6h-8c-50.4 0-99.1 18.1-137.2 51l-6.1 5.2zM191.6 187l178.8 207.2c38-30.5 84.8-47.9 133.5-49.6V70.9c-58.2 0.9-115.1 11.7-169.1 32.2-52.2 19.8-100.3 48-143.2 83.9z" fill="#0A0408"></path><path d="M472.3 315.8v-200c-87.6 0-167.6 32.1-229.1 85.2l130.7 151.5c26.4-22.9 60.8-36.7 98.4-36.7z" fill="#DC444A"></path><path d="M952.3 571.5c0-243.8-197.6-441.4-441.4-441.4" fill="#FFFFFF"></path><path d="M960.3 571.5h-16c0-2.6 0-5.3-0.1-7.9l16-0.3c0.1 2.7 0.1 5.4 0.1 8.2zM943.7 548.2c-0.3-5.1-0.6-10.3-1.1-15.4l15.9-1.4c0.5 5.3 0.8 10.7 1.1 16l-15.9 0.8z m-2.8-30.8c-0.6-5.1-1.4-10.2-2.2-15.3l15.8-2.5c0.8 5.3 1.6 10.6 2.3 15.9l-15.9 1.9z m-4.9-30.6c-1-5-2.1-10.1-3.3-15.1l15.6-3.7c1.2 5.2 2.4 10.5 3.4 15.7l-15.7 3.1z m-7.1-30.1c-1.4-5-2.8-9.9-4.4-14.8l15.3-4.8c1.6 5.1 3.1 10.2 4.5 15.4l-15.4 4.2z m-9.2-29.5c-1.7-4.8-3.5-9.7-5.4-14.5l14.9-5.9c2 5 3.9 10 5.6 15.1l-15.1 5.3z m-11.4-28.8c-2.1-4.7-4.2-9.5-6.4-14.1l14.4-6.9c2.3 4.8 4.5 9.7 6.7 14.6l-14.7 6.4z m-13.4-28c-2.4-4.5-4.9-9.1-7.4-13.6l13.9-7.9c2.7 4.6 5.2 9.4 7.7 14.1l-14.2 7.4z m-15.3-26.9c-2.7-4.4-5.5-8.7-8.4-13l13.3-8.9c3 4.4 5.9 9 8.7 13.5l-13.6 8.4z m-17.3-25.7c-3-4.2-6.1-8.3-9.3-12.4l12.6-9.8c3.3 4.2 6.5 8.5 9.6 12.8l-12.9 9.4z m-19-24.4c-3.3-3.9-6.7-7.9-10.1-11.7l11.9-10.7c3.6 3.9 7.1 8 10.5 12.1l-12.3 10.3z m-20.7-23c-3.6-3.7-7.2-7.4-10.9-10.9l11.1-11.5c3.8 3.7 7.6 7.5 11.3 11.3l-11.5 11.1zM800.4 249c-3.8-3.4-7.7-6.8-11.7-10.1l10.3-12.3c4.1 3.4 8.2 6.9 12.1 10.5L800.4 249z m-23.7-19.9c-4-3.1-8.2-6.3-12.4-9.3l9.4-13c4.3 3.1 8.6 6.3 12.8 9.6l-9.8 12.7z m-25.1-18c-4.3-2.8-8.6-5.7-13-8.4l8.4-13.6c4.5 2.8 9.1 5.7 13.5 8.7l-8.9 13.3z m-26.2-16.3c-4.4-2.5-9-5-13.5-7.4l7.4-14.2c4.7 2.5 9.4 5.1 14 7.7l-7.9 13.9zM698 180.5c-4.6-2.2-9.3-4.4-14-6.4l6.4-14.7c4.9 2.1 9.8 4.4 14.5 6.7l-6.9 14.4z m-28.2-12.4c-4.8-1.9-9.6-3.7-14.4-5.4l5.3-15.1c5 1.8 10 3.6 15 5.6l-5.9 14.9z m-29.1-10.2c-4.9-1.5-9.9-3-14.8-4.4l4.2-15.4c5.1 1.4 10.3 2.9 15.4 4.5l-4.8 15.3z m-29.8-8.2c-5-1.2-10.1-2.3-15.1-3.3l3.1-15.7c5.2 1 10.5 2.2 15.7 3.4l-3.7 15.6z m-30.4-6c-5.1-0.8-10.2-1.6-15.3-2.2l2-15.9c5.3 0.7 10.6 1.4 15.9 2.3l-2.6 15.8z m-30.6-3.9c-5.1-0.5-10.3-0.8-15.4-1.1l0.9-16c5.3 0.3 10.7 0.7 16 1.1l-1.5 16zM518.8 138.2c-2.6 0-5.2-0.1-7.9-0.1v-16c2.7 0 5.5 0 8.1 0.1l-0.2 16z" fill="#0A0408"></path><path d="M511.9 129.1v223.3C632.3 352.4 730 450 730 570.5h223.3" fill="#FFFFFF"></path><path d="M953.3 578.5H722v-8c0-115.8-94.2-210.1-210.1-210.1h-8V129.1h16v215.4c57.4 2 111 25.3 151.9 66.1s64.1 94.5 66.1 151.9h215.4v16z" fill="#0A0408"></path><path d="M953.8 593.5h-1c-12.5 0-22.6-10.1-22.6-22.6v-1c0-12.5 10.1-22.6 22.6-22.6h1c12.5 0 22.6 10.1 22.6 22.6v1c-0.1 12.5-10.2 22.6-22.6 22.6z" fill="#DC444A"></path><path d="M953.8 601.5h-1c-16.8 0-30.6-13.7-30.6-30.6v-1c0-16.8 13.7-30.6 30.6-30.6h1c16.8 0 30.6 13.7 30.6 30.6v1c-0.1 16.9-13.8 30.6-30.6 30.6z m-1-46.1c-8 0-14.6 6.5-14.6 14.6v1c0 8 6.5 14.6 14.6 14.6h1c8 0 14.6-6.5 14.6-14.6v-1c0-8-6.5-14.6-14.6-14.6h-1z" fill="#0A0408"></path><path d="M512.4 152.1h-1c-12.5 0-22.6-10.1-22.6-22.6v-1c0-12.5 10.1-22.6 22.6-22.6h1c12.5 0 22.6 10.1 22.6 22.6v1c-0.1 12.5-10.2 22.6-22.6 22.6z" fill="#DC444A"></path><path d="M512.4 160.1h-1c-16.8 0-30.6-13.7-30.6-30.6v-1c0-16.8 13.7-30.6 30.6-30.6h1c16.8 0 30.6 13.7 30.6 30.6v1c-0.1 16.9-13.8 30.6-30.6 30.6z m-1-46c-8 0-14.6 6.5-14.6 14.6v1c0 8 6.5 14.6 14.6 14.6h1c8 0 14.6-6.5 14.6-14.6v-1c0-8-6.5-14.6-14.6-14.6h-1z" fill="#0A0408"></path></g></svg>',
  'image/svg+xml',
  'image2.svg',
  1024,
  'hash_of_the_file',
  (SELECT id FROM users ORDER BY id LIMIT 1),
  NOW(),
  NOW()
);

INSERT INTO objects (team_id, object_name, object_data, mime_type, file_name, file_size, file_hash, created_by, created_at, updated_at)
VALUES (
  (SELECT id FROM teams ORDER BY id LIMIT 1),
  'example_object_name',
  '<svg height="200px" width="200px" version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <circle style="fill:#FF7F4F;" cx="256" cy="256" r="256"></circle> <path style="fill:#D35933;" d="M503.408,321.95L326.615,145.156l-20.035,37.076l-66.551-66.551l-37.755,43.494l12.06,12.06 l-46.325,15.725l88.302,88.302l-43.144,79.839l42.961,42.961l-0.126,9.904l88.369,88.369 C421.985,467.785,481.935,402.71,503.408,321.95z"></path> <polygon style="fill:#386895;" points="210.285,377.489 256,407.966 301.715,377.489 301.715,164.155 210.285,164.155 "></polygon> <polygon style="fill:#273B7A;" points="255.426,164.155 255.426,407.583 256,407.966 301.715,377.489 301.715,164.155 "></polygon> <path style="fill:#4A7EA8;" d="M271.972,115.681v38.964h-31.944v-38.964c-33.78,5.027-61.569,29.682-73.531,58.658 c-3.534,8.561,2.719,17.975,11.979,17.975h155.048c9.261,0,15.513-9.416,11.979-17.975 C333.541,145.361,305.752,120.706,271.972,115.681z"></path> <path style="fill:#386895;" d="M345.503,174.337c-11.962-28.975-39.753-53.631-73.531-58.658v38.965h-16.546v37.669h78.098 C342.785,192.314,349.037,182.898,345.503,174.337z"></path> <g> <polygon style="fill:#273B7A;" points="301.715,263.646 210.285,307.821 210.285,320.126 301.715,275.952 "></polygon> <polygon style="fill:#273B7A;" points="301.715,229.069 210.285,273.244 210.285,285.549 301.715,241.376 "></polygon> <polygon style="fill:#273B7A;" points="210.285,354.702 301.715,310.529 301.715,298.222 210.285,342.395 "></polygon> <polygon style="fill:#273B7A;" points="210.285,377.489 220.539,384.324 301.715,345.104 301.715,332.798 210.285,376.972 "></polygon> </g> <g> <polygon style="fill:#121149;" points="301.715,263.646 255.426,286.01 255.426,298.317 301.715,275.952 "></polygon> <polygon style="fill:#121149;" points="301.715,229.069 255.426,251.433 255.426,263.74 301.715,241.376 "></polygon> <polygon style="fill:#121149;" points="301.715,298.222 255.426,320.586 255.426,332.893 301.715,310.529 "></polygon> <polygon style="fill:#121149;" points="301.715,332.798 255.426,355.163 255.426,367.469 301.715,345.105 "></polygon> </g> </g></svg>',
  'image/svg+xml',
  'image3.svg',
  1024,
  'hash_of_the_file',
  (SELECT id FROM users ORDER BY id LIMIT 1),
  NOW(),
  NOW()
);

INSERT INTO objects (team_id, object_name, object_data, mime_type, file_name, file_size, file_hash, created_by, created_at, updated_at)
VALUES (
  (SELECT id FROM teams ORDER BY id LIMIT 1),
  'example_object_name',
  '<svg version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <circle style="fill:#C92F00;" cx="256" cy="256" r="256"></circle> <path style="fill:#930000;" d="M509.307,293.152L352.304,135.754l-107.279,9.007l-8.707-7.566h-52.722l-5.908,1.634l-6.754,1.689 l-11.174,14.138l13.807,14.317l-70.754,224.168l116.186,116.186c12.083,1.75,24.435,2.672,37.004,2.672 C384.765,512,491.31,416.927,509.307,293.152z"></path> <g> <rect x="152.462" y="219.608" style="fill:#FFFFFF;" width="207.076" height="25.626"></rect> <rect x="133.31" y="306.838" style="fill:#FFFFFF;" width="245.363" height="25.626"></rect> </g> <g> <rect x="256" y="219.608" style="fill:#D0D1D3;" width="103.538" height="25.626"></rect> <rect x="256" y="306.838" style="fill:#D0D1D3;" width="122.69" height="25.626"></rect> </g> <path style="fill:#FEE187;" d="M100.617,380.754l73.545-234.441c2-6.373,8.208-9.93,14.143-8.101l0,0 c6.418,1.977,10,9.438,7.839,16.331l-73.545,234.441c-2,6.373-8.208,9.93-14.143,8.101l0,0 C102.036,395.107,98.456,387.646,100.617,380.754z"></path> <g> <path style="fill:#FFC61B;" d="M411.383,380.754l-73.545-234.443c-2-6.373-8.208-9.93-14.143-8.101l0,0 c-6.418,1.977-10,9.438-7.839,16.331l73.545,234.441c2,6.373,8.208,9.93,14.143,8.101l0,0 C409.964,395.107,413.544,387.646,411.383,380.754z"></path> <path style="fill:#FFC61B;" d="M343.105,158.458H168.894c-7.116,0-12.884-5.768-12.884-12.884v-0.796 c0-7.116,5.768-12.884,12.884-12.884h174.211c7.116,0,12.884,5.768,12.884,12.884v0.796 C355.99,152.688,350.222,158.458,343.105,158.458z"></path> </g> <path style="fill:#EAA22F;" d="M343.105,131.889H256v26.567h87.105c7.116,0,12.884-5.768,12.884-12.884v-0.796 C355.99,137.659,350.222,131.889,343.105,131.889z"></path> </g></svg>',
  'image/svg+xml',
  'image4.svg',
  1024,
  'hash_of_the_file',
  (SELECT id FROM users ORDER BY id LIMIT 1),
  NOW(),
  NOW()
);

INSERT INTO objects (team_id, object_name, object_data, mime_type, file_name, file_size, file_hash, created_by, created_at, updated_at)
VALUES (
  (SELECT id FROM teams ORDER BY id LIMIT 1),
  'example_object_name',
  '<svg viewBox="0 0 64 64" data-name="Layer 1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"><defs><style> .cls-1 { fill: #f76c5e; } .cls-2 { fill: #e7ecef; } .cls-3 { fill: #8b8c89; } .cls-4 { fill: #bc6c25; } .cls-5 { fill: #a3cef1; } .cls-6 { fill: #dda15e; } .cls-7 { fill: #6096ba; } .cls-8 { fill: #274c77; } </style></defs><circle class="cls-5" cx="47" cy="17" r="13"></circle><path class="cls-6" d="M26.58,38.04l-3,1.29c-1.01,.43-2.15,.43-3.15,0l-3-1.29c-1.47-.63-2.42-2.08-2.42-3.68v-5.36c0-3.31,2.69-6,6-6h2c3.31,0,6,2.69,6,6v5.36c0,1.6-.95,3.05-2.42,3.68Z"></path><path class="cls-8" d="M35.14,44c-1.82-1.85-4.35-3-7.14-3h-3c0,1.66-1.34,3-3,3s-3-1.34-3-3h-3c-5.52,0-10,4.48-10,10v7H28l4-14h3.14Z"></path><path class="cls-6" d="M17,54h2c1.1,0,2,.9,2,2v2h-4v-4h0Z"></path><path class="cls-8" d="M12,30.73c-.29,.17-.64,.27-1,.27-1.1,0-2-.9-2-2s.9-2,2-2c.42,0,.81,.13,1.14,.36"></path><path class="cls-8" d="M32,30.73c.29,.17,.64,.27,1,.27,1.1,0,2-.9,2-2s-.9-2-2-2c-.42,0-.81,.13-1.14,.36"></path><path class="cls-4" d="M19,38.71l1.42,.61c1.01,.44,2.15,.44,3.16,0l1.42-.61v2.29c0,1.66-1.34,3-3,3s-3-1.34-3-3v-2.29Z"></path><polyline class="cls-3" points="28 58 32 44 54 44 50 58"></polyline><path class="cls-8" d="M28.4,26.38h-.01c-.57-.23-1.23-.38-2.06-.38-4.33,0-4.33,4-8.67,4-1.13,0-1.97-.27-2.66-.67v-.33c0-3.31,2.69-6,6-6h2c2.37,0,4.42,1.38,5.39,3.38h.01Z"></path><path class="cls-2" d="M29,33.6v-4.6c0-3.31-2.69-6-6-6h-2c-3.31,0-6,2.69-6,6v4h-2c-.55,0-1-.45-1-1v-3c0-5.52,4.48-10,10-10,2.76,0,5.26,1.12,7.07,2.93s2.93,4.31,2.93,7.07v3.18c0,.48-.34,.89-.8,.98l-2.2,.44Z"></path><path class="cls-5" d="M41,50c.55,0,1,.45,1,1s-.45,1-1,1v-2Z"></path><path class="cls-7" d="M22,35h0c-.11-.54,.24-1.07,.78-1.18l8.22-1.64v-2.86c0-4.79-3.61-8.98-8.38-9.3-5.24-.35-9.62,3.81-9.62,8.98v3h2v2h-2c-1.1,0-2-.9-2-2v-2.68c0-5.72,4.24-10.74,9.94-11.27,6.54-.62,12.06,4.53,12.06,10.95v3.18c0,.95-.67,1.77-1.61,1.96l-8.22,1.64c-.54,.11-1.07-.24-1.18-.78Z"></path><path class="cls-7" d="M22,58h-2v-2c0-.55-.45-1-1-1h-7c-.55,0-1-.45-1-1v-4c0-.27,.11-.52,.29-.71l1.29-1.29c.39-.39,1.02-.39,1.41,0h0c.39,.39,.39,1.02,0,1.41l-1,1v2.59h6c1.66,0,3,1.34,3,3v2Z"></path><rect class="cls-5" height="2" width="50" x="4" y="58"></rect><polygon class="cls-8" points="54 21 54 12 52 12 52 21 50 21 50 16 48 16 48 21 46 21 46 14 44 14 44 21 42 21 42 18 40 18 40 21 38 21 38 23 56 23 56 21 54 21"></polygon><path class="cls-1" d="M40.6,14.8l-1.2-1.6,4-3c.3-.23,.71-.26,1.05-.09l3.43,1.71,4.5-3.6,1.25,1.56-5,4c-.31,.24-.72,.29-1.07,.11l-3.45-1.72-3.51,2.63Z"></path><path class="cls-7" d="M44.29,29.72c-.19,.39-.29,.82-.29,1.28,0,1.66,1.34,3,3,3s3-1.34,3-3c0-.46-.1-.89-.29-1.28-.87,.18-1.78,.28-2.71,.28s-1.84-.1-2.71-.28Z"></path><circle class="cls-7" cx="44" cy="37" r="2"></circle><circle class="cls-7" cx="40" cy="41" r="2"></circle></g></svg>',
  'image/svg+xml',
  'image5.svg',
  1024,
  'hash_of_the_file',
  (SELECT id FROM users ORDER BY id LIMIT 1),
  NOW(),
  NOW()
);


-- Prepare an array containing prompt records
WITH prompts_array AS (
    SELECT * FROM (VALUES
        -- Prompt 1
        ('Welcome Prompt',                                   -- name
         'This prompt welcomes the user to the application. This prompt welcomes the user to the application. This prompt welcomes the user to the application. This prompt welcomes the user to the application. This prompt welcomes the user to the application.',-- description
         'Welcome to our application! How can I assist you today? Welcome to our application! How can I assist you today? Welcome to our application! How can I assist you today? Welcome to our application! How can I assist you today?', -- system_prompt
         ARRAY['How do I reset my password?',                -- example1
               'Where can I find tutorials?',                -- example2
               'What are the new features?',                 -- example3
               'How to contact support?'],                   -- example4
         'Productivity',                                     -- category_name
         'image0.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 2
        ('FAQ Prompt',                                       -- name
         'This prompt answers frequently asked questions.',  -- description
         'Here are some frequently asked questions.',        -- system_prompt
         ARRAY['What is the refund policy?',                 -- example1
               'How to upgrade my plan?',                    -- example2
               'Is there a mobile app?',                     -- example3
               'How to change my email address?'],           -- example4
         'Education',                                        -- category_name
         'image2.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 3
        ('Prompt Engineer',                                  -- name
         'Help with creating prompts for Generative AI applications.',      -- description
         'You are a specialist prompt engineer and your task is to improve entered prompts to get better results. You need to revise the entered prompt into a better prompt and then show the new prompt.',              -- system_prompt
         ARRAY['Setting up your profile.',                   -- example1
               'Creating your first project.',               -- example2
               'Collaborating with team members.',           -- example3
               'Exporting data.'],                           -- example4
         'Education',                                        -- category_name
         'image3.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 4
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image4.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 5
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image5.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 6
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image5.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 7
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image5.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 8
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image5.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 9
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image5.svg'                                        -- image_icon_object_id
        ),
        -- Prompt 10
        ('Feedback Prompt',                                  -- name
         'This prompt collects user feedback.',              -- description
         'We value your feedback. Please share your thoughts.', -- system_prompt
         ARRAY['The app is very user-friendly.',             -- example1
               'I would like more customization options.',   -- example2
               'The loading time is slow.',                  -- example3
               'Great customer support!'],                   -- example4
         'Lifestyle',                                        -- category_name
         'image5.svg'                                        -- image_icon_object_id
        )
    ) AS t(name, description, system_prompt, examples, category_name, image_icon_name)
)

-- Insert records from the array into the 'prompts' table
INSERT INTO prompts (
    team_id,
    model_id,
    visibility,
    name,
    max_history_items,
    max_chunks,
    max_tokens,
    trim_ratio,
    temperature,
    system_prompt,
    created_at,
    updated_at,
    created_by,
    prompt_type,
    description,
    disclaimer,
    example1,
    example2,
    example3,
    example4,
    category_id,
    image_icon_object_id
)
SELECT
    (SELECT id FROM teams ORDER BY id LIMIT 1),     -- First team_id
    (SELECT id FROM models ORDER BY id LIMIT 1),    -- First model_id
    'Company',                                      -- visibility
    name,                                           -- name from array
    10,                                             -- max_history_items
    5,                                              -- max_chunks
    1000,                                           -- max_tokens
    0.5,                                            -- trim_ratio
    0.7,                                            -- temperature
    system_prompt,                                  -- system_prompt from array
    NOW(),                                          -- created_at
    NOW(),                                          -- updated_at
    (SELECT id FROM users ORDER BY id LIMIT 1),     -- created_by
    'Assistant',                                    -- prompt_type
    description,                                    -- description from array
    'Please note that features may change over time.', -- disclaimer
    examples[1],                                    -- example1 from array
    examples[2],                                    -- example2 from array
    examples[3],                                    -- example3 from array
    examples[4],                                    -- example4 from array
    (SELECT id FROM categories WHERE name = category_name LIMIT 1), -- category_id from category_name
    (SELECT id FROM objects WHERE file_name = image_icon_name LIMIT 1)
FROM prompts_array;
