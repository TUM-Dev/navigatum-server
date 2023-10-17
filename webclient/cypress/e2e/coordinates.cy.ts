describe("Check if users can submit coordinates", () => {
  it("main page", () => {
    // 1902.02.286 is a unimportant room in Heilbron which will likely never get a coordinate
    cy.visit("http://localhost:8000/view/1902.02.286");
    cy.intercept("POST", "/api/feedback/get_token", { statusCode: 201, fixture: "feedback_token.json" });
  });
});

export {};
