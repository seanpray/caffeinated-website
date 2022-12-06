<script lang="ts">
    // string
    export let title;
    // unix
    export let date;
    // stuff
    export let text;
</script>

<section class="mt-10 font-semibold">
    <div class="flex flex-wrap">
        <div class="ml-20 text-[#fcebda] text-6xl">
            <div class="ml-3">
                {title}
            </div>
            <div class="flex w-[60rem]">
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
                <div class="dot2" />
            </div>
            <div class="mt-12 ml-20 text-[#fcebda] text-5xl">
                {text}
            </div>
        </div>
    </div>
</section>

<style>
	.dot2 {
		height: 1.3rem;
		width: 1.3rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 100;
		border-radius: 50%;
		margin: .9rem;
	}
</style>
