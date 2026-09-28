import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import svelte from '@astrojs/svelte';

export default defineConfig({
  site: 'https://reenuay.github.io',
  base: '/rust-game-development-lessons',
  integrations: [
    starlight({
      title: 'Rust Game Dev',
      defaultLocale: 'ru',
      locales: {
        ru: { label: 'Русский', lang: 'ru' },
      },
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/reenuay/rust-game-development-lessons' },
      ],
      sidebar: [
        {
          label: 'Уроки',
          items: [{ autogenerate: { directory: 'lessons' } }],
        },
      ],
    }),
    svelte(),
  ],
});
