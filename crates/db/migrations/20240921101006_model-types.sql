-- migrate:up
ALTER TYPE model_type ADD VALUE 'Image';
ALTER TYPE model_type ADD VALUE 'TextToSpeech';

-- migrate:down