import { writable } from 'svelte/store';

export const notes = writable([
    {
        id: 1,
        color: "orange",
        title: "First note's title",
        excerpt: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed dignissim consequat elementum.",
        starred: false,
    },
    {
        id: 2,
        color:"green",
        title:"Look at this!",
        excerpt:"Nam id diam eu arcu fringilla hendrerit at id lectus. Nullam vel posuere eros, at aliquet elit.",
        starred: true,
    },
    {
        id: 3,
        color: "blue",
        title: "This is very interesting",
        excerpt: "Praesent ante lectus, congue a ex vel, auctor condimentum tellus.",
        starred: false,
    },
    {
        id: 4,
        color: "pink",
        title: "OMG No WAY",
        excerpt: "Fusce erat lorem, molestie sed suscipit id, rhoncus vel arcu. Praesent nec neque turpis.",
        starred: false,
    }
]);
