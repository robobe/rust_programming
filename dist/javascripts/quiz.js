function bindQuizzes() {
  document.querySelectorAll(".quiz:not([data-quiz-bound])").forEach((quiz) => {
    quiz.dataset.quizBound = "true";
    quiz.querySelector(".quiz-check").addEventListener("click", () => {
      const answer = quiz.querySelector("input:checked");
      const result = quiz.querySelector(".quiz-result");
      if (!answer) {
        result.textContent = "Choose an answer first.";
        return;
      }
      result.textContent = answer.value === quiz.dataset.answer ? "Correct." : "Try again.";
    });
  });
}

if (typeof document$ === "undefined") bindQuizzes();
else document$.subscribe(bindQuizzes);
