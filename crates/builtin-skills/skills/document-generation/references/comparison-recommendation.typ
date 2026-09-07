#set page(
  paper: "a4",
  margin: 18mm,
)

#set text(
  size: 10.5pt,
)

= Comparison and Recommendation

#table(
  columns: (1.2fr, 3fr),
  inset: 5pt,
  stroke: 0.5pt,

  [*Decision*], [What is being compared],
  [*Prepared for*], [Decision-maker or stakeholder],
  [*Prepared by*], [Author or team],
  [*Date*], [Date],
)

== Decision Summary

State the decision to be made, the recommended option, and the main reason for the recommendation.

== Evaluation Criteria

#table(
  columns: (1.6fr, 3fr, 1fr),
  inset: 5pt,
  stroke: 0.5pt,

  [*Criterion*], [*What Good Looks Like*], [*Priority*],
  [Criterion 1], [Description of the requirement], [High],
  [Criterion 2], [Description of the requirement], [Medium],
  [Criterion 3], [Description of the requirement], [High],
)

== Options

#table(
  columns: (1.2fr, 2.2fr, 2.2fr, 2.2fr),
  inset: 5pt,
  stroke: 0.5pt,

  [*Criterion*], [*Option A*], [*Option B*], [*Option C*],
  [Criterion 1], [Assessment], [Assessment], [Assessment],
  [Criterion 2], [Assessment], [Assessment], [Assessment],
  [Criterion 3], [Assessment], [Assessment], [Assessment],
)

== Key Trade-offs

#table(
  columns: (1.3fr, 2.6fr, 2.6fr),
  inset: 5pt,
  stroke: 0.5pt,

  [*Option*], [*Strengths*], [*Risks / Limitations*],
  [Option A], [Main strengths], [Main risks or limitations],
  [Option B], [Main strengths], [Main risks or limitations],
  [Option C], [Main strengths], [Main risks or limitations],
)

== Recommendation

Recommend the preferred option. Tie the recommendation explicitly to the evaluation criteria, evidence, constraints, and important trade-offs.

== Decision Conditions

Document any assumptions, dependencies, unresolved questions, or conditions that could change the recommendation.

#table(
  columns: (2fr, 3fr),
  inset: 5pt,
  stroke: 0.5pt,

  [*Condition / Assumption*], [*Impact*],
  [Condition 1], [How it affects the decision],
  [Condition 2], [How it affects the decision],
)
