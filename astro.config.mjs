import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import svelte from '@astrojs/svelte';

export default defineConfig({
  site: 'https://reenuay.github.io',
  base: '/rust-game-development-lessons',
  integrations: [
    starlight({
      title: 'Rust Game Dev',
      defaultLocale: 'root',
      locales: {
        root: { label: 'Русский', lang: 'ru' },
      },
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/reenuay/rust-game-development-lessons' },
      ],
      components: {
        ThemeSelect: './src/components/ThemeSelect.astro',
        PageTitle: './src/components/PageTitle.astro',
      },
      customCss: ['./src/styles/custom.css', './src/styles/pulse.css'],
      sidebar: [
        {
          label: 'Подготовка',
          items: [{ autogenerate: { directory: 'lessons/setup' } }],
        },
        {
          label: 'Уроки',
          items: [{ autogenerate: { directory: 'lessons/basic' } }],
        },
        {
          label: 'Дополнительно',
          items: [{ autogenerate: { directory: 'lessons/extra' } }],
        },
      ],
    }),
    svelte(),
  ],
});
