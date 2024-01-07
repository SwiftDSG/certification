<template>
  <div class="rd-images" ref="rdImages" @click="timeoutHandler">
    <img
      class="rd-image"
      v-for="(image, i) in images"
      :key="i"
      :src="image"
      alt=""
    />
    <div
      v-if="images.length > 1"
      class="rd-image-indicators"
      ref="rdImageIndicators"
    >
      <div
        v-for="i in images.length"
        :key="i"
        class="rd-image-indicator"
        @click="switchTextures(i - 1)"
      >
        <div class="rd-image-indicator-inner"></div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import * as THREE from "three";
  import gsap from "gsap";

  const props = defineProps<{
    images: string[];
    state: "uninitialized" | "initialized";
  }>();
  const emits = defineEmits(["change"]);
  const config = useRuntimeConfig();

  const rdImages = ref<HTMLDivElement | null>(null);
  const rdImageIndicators = ref<HTMLDivElement | null>(null);

  const scene = ref<THREE.Scene | null>(null);
  const camera = ref<THREE.PerspectiveCamera | null>(null);
  const renderer = ref<THREE.WebGLRenderer | null>(null);
  const geometry = ref<THREE.PlaneGeometry | null>(null);
  const material = ref<THREE.ShaderMaterial | null>(null);
  const mesh = ref<THREE.Mesh | null>(null);
  const texture = ref<THREE.Texture[]>([]);

  const state = ref<["animating" | "idle", number]>(["idle", -1]);
  const timeout = ref<GSAPAnimation | null>(null);

  function loadTextures(): Promise<void> {
    return new Promise((resolve) => {
      if (rdImages.value) {
        const manager = new THREE.LoadingManager();
        const loader = new THREE.TextureLoader(manager);
        const rdImage: HTMLImageElement[] = gsap.utils.toArray(
          rdImages.value.querySelectorAll("img")
        );

        loader.load(`${config.public.base}/displacement.jpg`, (response) => {
          if (material.value) {
            material.value.uniforms.displacement.value = response;
          }
        });

        if (rdImage.length) {
          for (let i = 0; i < rdImage.length; i++) {
            loader.load(rdImage[i].src, (response) => {
              response.minFilter = THREE.LinearFilter;
              response.generateMipmaps = false;

              if (material.value) {
                material.value.uniforms.imageSize.value = [
                  rdImage[i].naturalWidth,
                  rdImage[i].naturalHeight,
                ];
                texture.value.push(response);
              }
              if (i === rdImage.length - 1) resolve();
            });
          }
        } else {
          resolve();
        }
      } else {
        resolve();
      }
    });
  }
  function switchTextures(index: number): void {
    if (state.value[0] === "animating") return;
    emits("change", index);
    state.value[0] = "animating";

    let rdImageIndicator: HTMLElement | undefined;

    if (rdImageIndicators.value) {
      rdImageIndicator = gsap.utils.toArray(
        rdImageIndicators.value.querySelectorAll(".rd-image-indicator-inner")
      )[state.value[1]] as HTMLElement;
    }

    if (rdImageIndicator && state.value[1] > -1) {
      if (timeout.value) timeout.value.kill();
      gsap.to(rdImageIndicator, {
        y: "100%",
        ease: "expo.inOut",
        duration: 0.5,
      });
    }

    if (rdImageIndicators.value) {
      rdImageIndicator = gsap.utils.toArray(
        rdImageIndicators.value.querySelectorAll(".rd-image-indicator-inner")
      )[index] as HTMLElement;
    }
    const tl = gsap.timeline({
      onComplete() {
        state.value[0] = "idle";
        state.value[1] = index;
        if (material.value) {
          material.value.uniforms.textureCurrent.value = texture.value[index];
        }
        if (rdImageIndicator) {
          timeout.value = gsap.fromTo(
            rdImageIndicator,
            {
              y: "-100%",
            },
            {
              y: 0,
              ease: "power0",
              duration: 5,
              onComplete() {
                switchTextures(
                  index === texture.value.length - 1 ? 0 : index + 1
                );
              },
            }
          );
        }
      },
    });

    if (material.value) {
      tl.fromTo(
        material.value.uniforms.progress,
        {
          value: 0,
        },
        {
          value: 1,
          duration: 2,
          ease: "expo.inOut",
          onStart() {
            if (material.value) {
              material.value.uniforms.textureNext.value = texture.value[index];
            }
          },
        }
      );
    }

    animate();
  }

  function timeoutHandler(): void {
    if (timeout.value) {
      if (timeout.value.isActive()) timeout.value.pause();
      else timeout.value.play();
    }
  }
  function animate(): void {
    if (
      state.value[0] === "animating" &&
      renderer.value &&
      scene.value &&
      camera.value
    ) {
      requestAnimationFrame(animate);
      renderer.value.render(scene.value.clone(), camera.value.clone());
    }
  }
  function initialize(): void {
    switchTextures(0);
    if (rdImageIndicators.value) {
      gsap.to(rdImageIndicators.value, {
        opacity: 1,
        duration: 0.25,
        delay: 1,
      });
    }
  }

  watch(
    () => props.state,
    (val) => {
      if (val === "initialized") initialize();
    }
  );

  onMounted(async () => {
    try {
      const promises = [
        fetch(`${config.public.base}/glsl/vertex.glsl`),
        fetch(`${config.public.base}/glsl/fragment.glsl`),
      ];

      const [vertexResponse, fragmentResponse] = await Promise.all(promises);
      if (
        vertexResponse.status === 200 &&
        fragmentResponse.status === 200 &&
        rdImages.value
      ) {
        const rect = rdImages.value.getBoundingClientRect();

        scene.value = new THREE.Scene();
        camera.value = new THREE.PerspectiveCamera(
          45,
          rect.width / rect.height,
          0.1,
          100
        );
        camera.value.position.z = 2;
        renderer.value = new THREE.WebGLRenderer();
        renderer.value.setSize(rect.width, rect.height);

        const rdCanvas = renderer.value.domElement;
        rdCanvas.classList.add("rd-canvas");
        rdImages.value.appendChild(rdCanvas);

        geometry.value = new THREE.PlaneGeometry(
          (rect.width / (rect.width + rect.width)) * 7.5,
          (rect.height / (rect.width + rect.width)) * 7.5,
          1,
          1
        );
        material.value = new THREE.ShaderMaterial({
          uniforms: {
            textureCurrent: { value: null },
            textureNext: { value: null },
            displacement: { value: null },
            meshSize: { value: [rect.width, rect.height] },
            imageSize: { value: [0, 0] },
            progress: { value: 0 },
          },
          vertexShader: await vertexResponse.text(),
          fragmentShader: await fragmentResponse.text(),
        });

        mesh.value = new THREE.Mesh(geometry.value, material.value);
        scene.value.add(mesh.value);

        await loadTextures();

        if (props.state === "initialized") {
          initialize();
        }
      } else {
        throw new Error("GLSL_NOT_FOUND");
      }
    } catch (e) {
      console.log(e);
    }
  });
</script>

<style lang="scss" scoped>
  .rd-images {
    position: relative;
    width: 100%;
    height: 100%;
    > * {
      pointer-events: none;
    }
    img.rd-image {
      position: absolute;
      top: 0;
      left: 0;
      max-width: 100%;
      height: 100%;
      object-fit: cover;
      display: none;
    }
    .rd-image-indicators {
      pointer-events: all;
      position: absolute;
      right: 1rem;
      top: 50%;
      transform: translateY(-50%);
      display: flex;
      flex-direction: column;
      justify-content: center;
      gap: 0.5rem;
      opacity: 0;
      .rd-image-indicator {
        cursor: pointer;
        position: relative;
        width: 0.25rem;
        height: 1.5rem;
        border-radius: 0.125rem;
        overflow: hidden;
        display: flex;

        .rd-image-indicator-inner {
          pointer-events: none;
          position: relative;
          width: 100%;
          height: 100%;
          background: var(--font-sub-color);
          transform: translateY(-100%);
        }

        &::before {
          content: "";
          position: absolute;
          width: 100%;
          height: 100%;
          background: var(--font-sub-color);
          opacity: 0.5;
        }
      }
    }
    @media only screen and (max-width: 1024px) {
      .rd-image-indicators {
        right: 0.5rem;
        gap: 0.25rem;
        .rd-image-indicator {
          height: 1rem;
        }
      }
    }
  }
</style>
