<template>
  <span class="rd-image-wrapper" ref="rdImageWrapper">
    <span class="rd-image-container rd-image-container-right">
      <img :src="data" class="rd-image" rd-parallax alt="" />
    </span>
  </span>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    data: string;
    state: "uninitialized" | "initialized";
  }>();

  const rdImageWrapper = ref<HTMLDivElement | null>(null);

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized" && rdImageWrapper.value) {
        animate.init(rdImageWrapper.value);
      }
    }
  );

  const animate = {
    init(rdImageWrapper: HTMLElement): void {
      const tl = gsap.timeline();

      const rdImageContainer: HTMLElement[] = gsap.utils.toArray(
        rdImageWrapper.querySelectorAll(".rd-image-container")
      );
      const rdImage: HTMLElement[] = gsap.utils.toArray(
        rdImageWrapper.querySelectorAll(".rd-image")
      );

      tl.to(rdImageContainer, {
        x: 0,
        duration: 2,
        ease: "expo.inOut",
      }).to(
        rdImage,
        {
          x: 0,
          scale: 1.25,
          duration: 2,
          ease: "expo.inOut",
        },
        "<0"
      );
    },
  };

  onMounted(() => {
    if (props.state === "initialized" && rdImageWrapper.value) {
      animate.init(rdImageWrapper.value);
    }
  });
</script>
