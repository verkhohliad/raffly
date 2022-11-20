const logger = {
  log: (...messages) => {
    console.log(...messages);
  },
  error: (...errorMessages) => {
    console.error(errorMessages);
  },
  warn: (...warnMessages) => {
    console.warn(...warnMessages);
  }
}

export default logger;
