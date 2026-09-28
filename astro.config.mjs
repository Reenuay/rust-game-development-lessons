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
      },
      customCss: ['./src/styles/custom.css'],
      sidebar: [
        {
          label: 'Подготовка',
          items: [{ autogenerate: { directory: 'lessons/setup' } }],
        },
        {
          label: 'Базовый уровень',
          items: [{ autogenerate: { directory: 'lessons/basic' } }],
        },
        {
          label: 'Средний уровень',
          items: [{ autogenerate: { directory: 'lessons/medium' } }],
        },
        {
          label: 'Сложный уровень',
          items: [{ autogenerate: { directory: 'lessons/hard' } }],
        },
      ],
    }),
    svelte(),
  ],
});
