const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],

	plugins: [require('flowbite/plugin'), require('@tailwindcss/typography')],

	darkMode: 'class',

	theme: {
		extend: {
			colors: {
				// flowbite-svelte
				primary: {
					50: '#FFF5F2',
					100: '#FFF1EE',
					200: '#FFE4DE',
					300: '#FFD5CC',
					400: '#FFBCAD',
					500: '#FE795D',
					600: '#EF562F',
					700: '#EB4F27',
					800: '#CC4522',
					900: '#A5371B'
				},
				main: '#e0ecf5',
				// note colors
				'note-orange': '#fdba8c', // orange-300
				'note-green': '#84e1bc', // green-300
				'note-blue': '#a4cafe', // blue-300
				'note-pink': '#f8b4d9', // pink-300

				dark: {
					main: '#1a202c'
				}
			},
			screens: {
				'3xl': '1792px'
			}
		}
	}
};

module.exports = config;
