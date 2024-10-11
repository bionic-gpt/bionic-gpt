-- migrate:up
UPDATE prompts
SET 
    example1 = 'What is the capital of France?',
    example2 = 'How does machine learning work?',
    example3 = 'Morning routine for productivity',
    example4 = 'Can you explain recursion?'
WHERE 
    (example1 IS NULL OR example1 = '') AND
    (example2 IS NULL OR example2 = '') AND
    (example3 IS NULL OR example3 = '') AND
    (example4 IS NULL OR example4 = '');

-- migrate:down

