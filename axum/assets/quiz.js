/**
 * Axum 0.8 Workspace — Shared Quiz Widget
 *
 * Markup (zero-indexed answer on the question container):
 *
 * <div class="quiz-section">
 *   <div class="quiz-question" data-correct="2">
 *     <p class="quiz-prompt">What is the thing?</p>
 *     <button class="quiz-option">Option A</button>
 *     <button class="quiz-option">Option B</button>
 *     <button class="quiz-option">Option C</button>
 *     <button class="quiz-option">Option D</button>
 *     <p class="quiz-explanation">Shown after the learner answers.</p>
 *   </div>
 * </div>
 *
 * Feedback is immediate and automatic. Each question locks after one answer.
 */
(function () {
  function init() {
    document.querySelectorAll('.quiz-question').forEach(function (q) {
      var correctIdx = parseInt(q.getAttribute('data-correct'), 10);
      var options = Array.prototype.slice.call(q.querySelectorAll('.quiz-option'));
      var explanation = q.querySelector('.quiz-explanation');
      if (explanation) explanation.style.display = 'none';

      options.forEach(function (btn, idx) {
        btn.addEventListener('click', function () {
          if (q.classList.contains('answered')) return;
          q.classList.add('answered');

          options.forEach(function (b, i) {
            b.disabled = true;
            if (i === correctIdx) {
              b.classList.add('correct');
            } else {
              b.classList.add('incorrect');
            }
          });

          if (idx === correctIdx) {
            btn.classList.add('selected-correct');
          } else {
            btn.classList.add('selected-incorrect');
          }

          if (explanation) explanation.style.display = 'block';
        });
      });
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
