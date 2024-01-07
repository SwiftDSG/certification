<template>
  <div
    class="rd-product"
    ref="rdProduct"
    @mouseenter="hovered = true"
    @mouseout="hovered = false"
  >
    <div v-if="product.image" class="rd-product-image-container">
      <span class="rd-image-wrapper">
        <span class="rd-image-container rd-image-container-down">
          <img
            :src="product.image"
            class="rd-image"
            :alt="`Item ${product.name}`"
          />
        </span>
      </span>
    </div>
    <div class="rd-product-detail-container">
      <div class="rd-product-name-container">
        <div class="rd-product-name-ripple"></div>
        <span class="rd-product-name rd-subtitle-text">
          {{ product.name }}
        </span>
        <rd-input-button-attraction-small
          @clicked="redirect"
          class="rd-product-button"
          icon="arrow-top-right"
          type="secondary"
          state="initialized"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";
  import { ProductMin } from "types/product";

  const props = defineProps<{
    product: ProductMin;
    state: "uninitialized" | "initialized";
  }>();

  const hovered = ref<boolean>(false);

  const rdProduct = ref<HTMLDivElement | null>(null);

  const animate = {
    init(rdProduct: HTMLElement): void {
      const tl = gsap.timeline({
        onComplete() {
          gsap.to(rdProduct, {
            pointerEvents: "all",
            duration: 0,
          });
          gsap.to(rdImage, {
            transition: "transform 1s cubic-bezier(0.16, 1, 0.3, 1)",
            duration: 0,
          });
        },
      });

      const rdImageContainer: HTMLElement[] = gsap.utils.toArray(
        rdProduct.querySelectorAll(".rd-image-container")
      );
      const rdImage: HTMLElement[] = gsap.utils.toArray(
        rdProduct.querySelectorAll(".rd-image")
      );

      tl.to(
        rdImageContainer,
        {
          y: 0,
          duration: 1,
          ease: "expo.inOut",
          stagger: 0.1,
        },
        "<0"
      ).fromTo(
        rdImage,
        {
          scale: 1.5,
        },
        {
          y: 0,
          scale: 1.25,
          duration: 1,
          ease: "expo.inOut",
          stagger: 0.1,
        },
        "<0"
      );
    },
  };

  function redirect(): void {
    window.location.href = "https://redian.id/incarnaxion-creator";
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized" && rdProduct.value) {
        animate.init(rdProduct.value);
      }
    }
  );

  onMounted(() => {
    if (props.state === "initialized" && rdProduct.value) {
      animate.init(rdProduct.value);
    }
  });
</script>

<style lang="scss" scoped>
  .rd-product {
    pointer-events: none;
    position: relative;
    width: 100%;
    aspect-ratio: 1 / 1;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    .rd-product-image-container {
      pointer-events: none;
      position: absolute;
      width: 100%;
      height: 100%;
      border-radius: 1rem;
      display: flex;
      overflow: hidden;
    }
    .rd-product-detail-container {
      position: absolute;
      width: 100%;
      height: 100%;
      border-radius: 1rem;
      display: flex;
      overflow: hidden;
      .rd-product-name-container {
        position: absolute;
        bottom: 0;
        width: 100%;
        height: 3rem;
        .rd-product-name-ripple {
          position: absolute;
          width: 100%;
          height: 100%;
          border-radius: 50% 50% 0 0;
          background: var(--font-primary-color);
          transform: translateY(101%);
          transition: transform 0.5s cubic-bezier(0.4, 0, 0, 1),
            border-radius 0.5s cubic-bezier(0.4, 0, 0, 1);
        }
        span.rd-product-name {
          position: absolute;
          bottom: 0;
          left: 0;
          width: 100%;
          height: 100%;
          color: var(--font-secondary-color);
          padding: 0 1rem;
          box-sizing: border-box;
          display: flex;
          align-items: center;
          transform: translateY(110%);
          transition: transform 1s cubic-bezier(0.16, 1, 0.3, 1);
        }
        .rd-product-button {
          position: absolute;
          bottom: 1.5rem;
          right: 1.5rem;
          opacity: 0 !important;
          transition: 0.25s opacity linear;
        }
      }
    }
    &:hover,
    &:focus-within {
      .rd-product-image-container {
        img {
          transform: scale(1, 1) !important;
        }
      }
      .rd-product-detail-container {
        .rd-product-name-container {
          .rd-product-name-ripple {
            border-radius: 0;
            transform: translateY(0);
            transition-duration: 0.5s, 0.9s;
          }
          span.rd-product-name {
            transform: translateY(0);
          }
          .rd-product-button {
            opacity: 1 !important;
          }
        }
      }
    }
  }
  html[dir="rtl"] {
    .rd-product {
      .rd-product-name-container {
        span.rd-product-name {
          left: auto;
          right: 0;
        }
        .rd-product-button {
          left: 1.5rem;
          right: auto;
        }
      }
    }
  }
</style>
