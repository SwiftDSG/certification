export default defineNuxtPlugin(() => {
  const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  return {
    provide: {
      formatDateTime: (x: string | number): string => {
        const date = new Date(x);
        return `${date.getDate().toString().padStart(2, "0")}/${(
          date.getMonth() + 1
        )
          .toString()
          .padStart(2, "0")}/${date.getFullYear()} ${date
          .getHours()
          .toString()
          .padStart(2, "0")}:${date.getMinutes().toString().padStart(2, "0")}`;
      },
      formatDate: (x: string | number): string => {
        const date = new Date(x);
        return `${date.getDate().toString().padStart(2, "0")} ${months[
          date.getMonth()
        ].slice(0, 3)} ${date.getFullYear()}`;
      },
      formatSize(b: number): string {
        if (b === 0) return "0 Byte";

        const i = Math.floor(Math.log(b) / Math.log(1024));
        const size = b / Math.pow(1024, i);

        return `${size.toFixed(2)} ${sizes[i]}`;
      },
    },
  };
});
