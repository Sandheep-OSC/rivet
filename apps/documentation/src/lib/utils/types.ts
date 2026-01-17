export interface Doc {
  slug: string;
  title: string;
  description?: string;
  section: string;
  order?: number;
  content?: string;
}

export interface SearchResult {
  slug: string;
  title: string;
  description?: string;
  section?: string;
  content: string;
  _formatted?: {
    title?: string;
    description?: string;
  };
}
