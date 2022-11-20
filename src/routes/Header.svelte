<script>
	import { page } from '$app/stores';
    import logo from "$lib/images/logo.png";
	import github from '$lib/images/github.svg';
    import { browser } from '$app/environment';
    let mobile = false;
    const MOBILE_WIDTH = 1400;
    const handleResize = (e) => {
		mobile = window.innerWidth < MOBILE_WIDTH;
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
		window.addEventListener('resize', debounce(handleResize, 50));
	}
</script>

<header class="bg-[#f4eae1]">
        <div class="corner">
            {#if !mobile}
                <a href="https://kit.svelte.dev">
                    <img src={logo} alt="logo" />
                </a>
            {/if}
        </div>

	<nav>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
		</svg>
		<ul>
			<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
				<a href="/">Home</a>
			</li>
			<li aria-current={$page.url.pathname === '/robot' ? 'page' : undefined}>
				<a href="/robot">Robot</a>
			</li>
			<li aria-current={$page.url.pathname.startsWith('/blog') ? 'page' : undefined}>
				<a href="/blog">Blog</a>
			</li>
			<li aria-current={$page.url.pathname.startsWith('/sponsors') ? 'page' : undefined}>
				<a href="/sponsors">Sponsors</a>
			</li>
		</ul>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
		</svg>
	</nav>

    <div class="corner">
        {#if !mobile}
            <a href="https://github.com/sveltejs/kit">
                <img src={github} alt="GitHub" />
            </a>
        {/if}
    </div>
</header>

<style>
</style>
