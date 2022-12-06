<script>
	import { page } from '$app/stores';
    import logo from "$lib/images/logo.png";
	import github from '$lib/images/github.svg';
</script>

<header class="flex justify-center text-center lowercase">
    <nav>
        <ul>
            <li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
                <a class="text-white" href="/">about</a>
            </li>
            <li aria-current={$page.url.pathname.startsWith('/media') ? 'page' : undefined}>
                <a class="text-white" href="/media">media</a>
            </li>
            <li aria-current={$page.url.pathname.startsWith('/blog') ? 'page' : undefined}>
                <a class="text-white" href="/blog">blog</a>
            </li>
            <li aria-current={$page.url.pathname.startsWith('/robot') ? 'page' : undefined}>
                <a class="text-white" href="/robot">Robot</a>
            </li>
            <li aria-current={$page.url.pathname.startsWith('/sponsors') ? 'page' : undefined}>
                <a class="text-white" href="/sponsors">Sponsors</a>
            </li>
            <!-- <li aria-current={$page.url.pathname.startsWith('/member') ? 'page' : undefined}> -->
            <!-- 	<a class="text-white" href="/member">Members</a> -->
            <!-- </li> -->
        </ul>
    </nav>
</header>
