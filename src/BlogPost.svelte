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
			<div class="flex dot-container">
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
	.dot-container {
		width: 40rem;
	}
	.dot2 {
		height: 1.1rem;
		width: 1.1rem;
		border: 1px solid #bc8f62;
		background: #bc8f62;
		z-index: 100;
		border-radius: 50%;
		margin: 0.7rem;
	}
	@media (max-width: 1500px) {
		.dot2 {
			height: 1vw;
			width: 1vw;
			border: 1px solid #bc8f62;
			background: #bc8f62;
			z-index: 100;
			border-radius: 50%;
			margin: 0.8vw;
		}
		.dot-container {
			width: 50vw;
		}
	}
</style>
