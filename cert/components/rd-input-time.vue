<template>
  <div class="rd-component">
    <label v-if="input.label" class="rd-input-label rd-headline-6">{{
      input.label
    }}</label>
    <div class="rd-input-wrapper">
      <rd-input-text class="rd-input" :input="hoursInput" />
      <div class="rd-input-divider"></div>
      <rd-input-text class="rd-input" :input="minutesInput" />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import { InputOption, InputTimeOption } from "~~/types/general";

  const props = defineProps<{
    input: InputTimeOption;
  }>();

  const hoursInput = ref<InputOption>({
    name: "hours",
    placeholder: "HH",
    model: "00",
    type: "hour",
  });
  const minutesInput = ref<InputOption>({
    name: "minutes",
    placeholder: "MM",
    model: "00",
    type: "minute",
  });

  const time = computed<[number, number]>(() => [
    parseInt(hoursInput.value.model) || 0,
    parseInt(minutesInput.value.model) || 0,
  ]);

  watch(
    () => props.input.model,
    (val) => {
      if (
        val.length === 2 &&
        (val[0] !== time.value[0] || val[1] !== time.value[1])
      ) {
        if (val[0] >= 0 && val[0] <= 23)
          hoursInput.value.model = val[0].toString();
        if (val[1] >= 0 && val[1] <= 59)
          minutesInput.value.model = val[1].toString();
      }
    }
  );
  watch(
    () => time.value,
    (val) => {
      if (val[0] >= 0 && val[0] <= 23) props.input.model[0] = val[0];
      if (val[1] >= 0 && val[1] <= 59) props.input.model[1] = val[1];
    },
    {
      deep: true,
    }
  );

  onMounted(() => {
    if (props.input.model) {
      if (props.input.model[0] >= 0 && props.input.model[0] <= 23)
        hoursInput.value.model = props.input.model[0].toString();
      if (props.input.model[1] >= 0 && props.input.model[1] <= 59)
        minutesInput.value.model = props.input.model[1].toString();
    }
  });
</script>

<style lang="scss" scoped>
  .rd-component {
    position: relative;
    width: 100%;
    display: flex;
    flex-direction: column;
    label.rd-input-label {
      position: relative;
      width: 100%;
      height: 1rem;
      display: flex;
      color: var(--font-primary-color);
      opacity: 0.5;
      align-items: center;
    }
    .rd-input-wrapper {
      position: relative;
      width: 100%;
      display: flex;
      .rd-input-divider {
        position: relative;
        width: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.125rem;
        justify-content: center;
        align-items: center;
        &::before {
          content: "";
          position: relative;
          width: 0.125rem;
          height: 0.125rem;
          background: var(--border-color);
          border-radius: 50%;
        }
        &::after {
          content: "";
          position: relative;
          width: 0.125rem;
          height: 0.125rem;
          background: var(--border-color);
          border-radius: 50%;
        }
      }
      .rd-input {
        position: relative;
        width: calc((100% - 0.75rem) / 2);
      }
    }
  }
</style>
