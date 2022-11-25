<script lang="ts">
    export let image;
    export let name;
</script>

<main>
    <img class="rounded-md h-52 m-4" src={image} alt="" />
    <p>{name}</p>
</main>
