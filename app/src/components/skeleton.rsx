<template>
    <div>
        <div id="skeleton" class="skeleton">
            <div class="skeleton__header"></div>
            <div class="skeleton__content"></div>
        </div>
        <slot></slot>
    </div>
</template>

<script>
    import { onMounted } from 'rsx';

    onMounted(() => {
        const skeletonRef = document.getElementById('skeleton');
        skeletonRef.style.display = 'none';
    })
</script>

<style>
    .skeleton {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .skeleton__header {
        width: 100%;
        height: 50px;
        background-color: #f0f0f0;
    }

    .skeleton__content {
        width: 100%;
        height: 250px;
        background-color: #f0f0f0;
    }
</style>
