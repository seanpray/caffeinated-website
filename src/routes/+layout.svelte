<script lang="ts">
	import Header from './Header.svelte';
	import './styles.scss';
	import { onMount } from 'svelte';
    import { type AuthData, loggedin, ismobile } from "../store.ts";
	import ls from 'localstorage-slim';
    import { browser } from '$app/environment';

    import linkedin from "$lib/icons/linked.png";
    import gh from "$lib/icons/github.webp";
    import mail from "$lib/icons/mail.png";

    let mobile = false;
    const MOBILE_WIDTH = 1000;
    const handleResize = (e) => {
		mobile = window.innerWidth < MOBILE_WIDTH;
        ismobile.set(mobile);
	};
	if (browser) {
		handleResize();
		const debounce = (fn, interval) => {
			let timer;
			return function debounced(...args) {
				clearTimeout(timer);
				timer = setTimeout(function call() {
					fn(...args);
				}, interval);
			};
		};
		window.addEventListener('resize', debounce(handleResize, 2));
	}

	onMount(() => {
		if (ls.get('auth', { decrypt: true })) {
            try {

            } catch {
                console.log("parse fail")
                clear_login();
            }
		} else {
			console.log('not auth');
            console.log('auth');
		}
		// get logged in
	});
</script>

<div class="bg-[#2d1902] min-h-screen">
    <Header />
    <slot />
    <footer class="flex text-center justify-center py-20">
        <!-- <p class="text-red-100">visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p> -->
        <a
            class="m-2 p-1 tracking-widest rounded-md hover:bg-[#bc8f62] duration-300"
            href="https://www.linkedin.com/company/frc9293/"
            target="_blank"
            rel="noopener noreferrer"
        >
            <img class="h-12 w-12" src={linkedin} alt="" />
        </a>
        <a
            class="m-2 p-1 tracking-widest rounded-md hover:bg-[#bc8f62] duration-300"
            href="https://github.com/notseanray/caffeinated-website"
            target="_blank"
            rel="noopener noreferrer"
        >
            <img class="h-12 w-12" src={gh} alt="" />
        </a>
        <a
            class="m-2 p-1 tracking-widest rounded-md hover:bg-[#bc8f62] duration-300"
            href="mailto:caffeinated9293@gmail.com"
            target="_blank"
            rel="noopener noreferrer"
        >
            <img class="h-12 w-12" src={mail} alt="" />
        </a>
    </footer>
</div>
