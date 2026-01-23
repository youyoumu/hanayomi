/**
 * Database table types for dictionary data
 * Represents core database entities and their relationships
 */

import { Definition } from "../schemas/dictionary_term_bank_v3_types.ts";

/**
 * Main dictionary table structure
 */
export interface Dictionary {
  /** Unique identifier for the dictionary */
  id: number;
  /** Timestamp when dictionary was created */
  createdAt: string;
  /** Timestamp when dictionary was last updated */
  updatedAt: string;
  /** Dictionary title */
  title: string;
  /** Dictionary version/revision */
  revision: string;
  /** Author information */
  author?: string | null;
  /** Dictionary description */
  description?: string | null;
  /** Attribution information */
  attribution?: string | null;
  /** Source URL for dictionary */
  url?: string | null;

  // Core data settings
  /** Source language of dictionary terms */
  sourceLanguage?: string | null;
  /** Target language for dictionary terms */
  targetLanguage?: string | null;
  /** Frequency mode for dictionary entries */
  frequencyMode?: string | null;

  // Yomitan specific settings
  /** Format version (resolved from 'format' or 'version') */
  format: number;
  /** Whether dictionary is sequenced */
  sequenced: boolean;
  /** Minimum Yomitan version requirement */
  minimumYomitanVersion?: string | null;

  // Update information
  /** Whether dictionary supports updates */
  isUpdatable: boolean;
  /** URL for dictionary index */
  indexUrl?: string | null;
  /** URL for dictionary downloads */
  downloadUrl?: string | null;

  // Metadata stored as JSON
  /** Tag metadata in JSON format */
  tagMeta?: Record<string, any> | null;
}

/**
 * Individual dictionary entry/term
 */
export interface DictionaryEntry {
  /** Unique identifier for the entry */
  id: number;
  /** Timestamp when entry was created */
  createdAt: string;
  /** ID of the parent dictionary */
  dictionaryId: number;

  // Core term data
  /** The expression/text of the term */
  expression: string;
  /** Reading pronunciation of the term */
  reading: string;
  /** Array of definitions for this term */
  definitions: Definition[];
  /** Inflection rules for the term */
  rules: string;
  /** Relevance score for search ranking */
  score: number;
  /** Sequence number for grouping related terms */
  sequence: number;
  /** Tags associated with the definition */
  definitionTags: string;
  /** Tags associated with the expression */
  expressionTags: string;
}

/**
 * Definition tag/category metadata
 */
export interface DefinitionTag {
  /** Unique identifier for the tag */
  id: number;
  /** Timestamp when tag was created */
  createdAt: string;
  /** ID of the parent dictionary */
  dictionaryId: number;

  /** Tag name */
  name: string;
  /** Category for organizing tags */
  category: string;
  /** Sort order for tags */
  order: number;
  /** Additional notes about the tag */
  notes: string;
  /** Relevance score for the tag */
  score: number;
}

