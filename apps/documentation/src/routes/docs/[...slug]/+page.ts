import { error } from '@sveltejs/kit';
import type { EntryGenerator, PageLoad } from './$types';

// Placeholder entries - will be replaced when we add actual docs
export const entries: EntryGenerator = async () => {
  return [{ slug: 'intro' }, { slug: 'api' }, { slug: 'guides' }];
};

export const load: PageLoad = async ({ params }) => {
  const slug = params.slug;

  // For now, return null content - will be replaced with actual MDX loading
  // When docs are added, this will load from $lib/docs/*.md files
  const modules = import.meta.glob('$lib/docs/*.md', { eager: true });

  for (const [path, module] of Object.entries(modules)) {
    const match = path.match(/\/([^/]+)\.md$/);
    if (match && match[1] === slug) {
      return {
        content: module,
        slug,
        previous: null, // Will be calculated based on doc order
        next: null,
      };
    }
  }

  // For now, show placeholder content for known routes
  if (['intro', 'api', 'guides'].includes(slug)) {
    return {
      content: null,
      slug,
      previous: null,
      next: null,
    };
  }

  throw error(404, `Documentation page not found: ${slug}`);
};
