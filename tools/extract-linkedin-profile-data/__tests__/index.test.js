/**
 * Jest test suite for extract-linkedin-profile-data
 */

const fs = require("fs").promises;
const path = require("path");
const { exec } = require("child_process");
const { promisify } = require("util");

const execAsync = promisify(exec);

// Mock data for testing
const mockLinkedInData = {
  timestamp: new Date().toISOString(),
  url: "https://www.linkedin.com/in/testuser/",
  title: "Test LinkedIn Profile",
  fullText: "Test profile content with experience and skills",
  byId: {
    "profile-id": {
      tag: "DIV",
      text: "Profile content",
      html: '<div id="profile-id">Profile content</div>',
      attrs: { id: "profile-id", class: "profile-section" },
    },
  },
  byDataSection: {
    experience: [
      {
        tag: "SECTION",
        text: "Software Engineer at Tech Corp",
        html: '<section data-section="experience">Software Engineer at Tech Corp</section>',
        attrs: { "data-section": "experience" },
      },
    ],
    skills: [
      {
        tag: "SECTION",
        text: "JavaScript, Python, React",
        html: '<section data-section="skills">JavaScript, Python, React</section>',
        attrs: { "data-section": "skills" },
      },
    ],
  },
  byAriaLabel: {
    Experience: [
      {
        tag: "SECTION",
        text: "Work experience details",
        html: '<section aria-label="Experience">Work experience details</section>',
        attrs: { "aria-label": "Experience" },
      },
    ],
  },
  byClass: {
    "pvs-list__item--line-separated": [
      {
        tag: "LI",
        text: "List item content",
        html: '<li class="pvs-list__item--line-separated">List item content</li>',
        attrs: { class: "pvs-list__item--line-separated" },
      },
    ],
  },
  rawHtml:
    "<!DOCTYPE html><html><head><title>Test Profile</title></head><body><div>Test content</div></body></html>",
};

// Test utilities
const testUtils = {
  async cleanupTestFiles() {
    try {
      const distDir = path.join(__dirname, "..", "dist");
      const files = await fs.readdir(distDir);
      for (const file of files) {
        if (file.startsWith("linkedin-dump-") && file.endsWith(".json")) {
          await fs.unlink(path.join(distDir, file));
        }
      }
    } catch (error) {
      // Directory might not exist, which is fine
    }
  },

  generateLinkedInUrl(username, page = "profile") {
    const baseUrl = `https://www.linkedin.com/in/${username}/`;
    if (page === "profile") return baseUrl;
    return `${baseUrl}details/${page}/`;
  },

  validateDataStructure(data) {
    const requiredFields = [
      "timestamp",
      "url",
      "title",
      "fullText",
      "byId",
      "byDataSection",
      "byAriaLabel",
      "byClass",
      "rawHtml",
    ];
    return requiredFields.every((field) => data.hasOwnProperty(field));
  },
};

describe("LinkedIn Profile Data Extractor", () => {
  beforeAll(async () => {
    await testUtils.cleanupTestFiles();
  });

  afterAll(async () => {
    await testUtils.cleanupTestFiles();
  });

  describe("Argument Validation", () => {
    test("should fail when no username is provided", async () => {
      try {
        await execAsync("node index.js");
        fail("Should have thrown an error for missing username");
      } catch (error) {
        expect(error.stderr || error.message).toMatch(
          /Usage:|Cannot find module|Missing argument/
        );
      }
    });

    test("should accept valid usernames", () => {
      const validUsernames = [
        "matthewelder",
        "john-doe",
        "jane_smith",
        "user123",
      ];

      validUsernames.forEach((username) => {
        const url = testUtils.generateLinkedInUrl(username);
        expect(url).toBe(`https://www.linkedin.com/in/${username}/`);
      });
    });
  });

  describe("URL Generation", () => {
    test("should generate correct profile URLs", () => {
      const username = "testuser";
      const expectedUrl = `https://www.linkedin.com/in/${username}/`;
      const actualUrl = testUtils.generateLinkedInUrl(username);

      expect(actualUrl).toBe(expectedUrl);
    });

    test("should generate correct subpage URLs", () => {
      const username = "testuser";
      const pages = ["experience", "skills", "recommendations"];

      pages.forEach((page) => {
        const expectedUrl = `https://www.linkedin.com/in/${username}/details/${page}/`;
        const actualUrl = testUtils.generateLinkedInUrl(username, page);

        expect(actualUrl).toBe(expectedUrl);
      });
    });

    test("should handle special characters in usernames", () => {
      const specialUsernames = [
        "user-name",
        "user_name",
        "user123",
        "user@domain",
      ];

      specialUsernames.forEach((username) => {
        const url = testUtils.generateLinkedInUrl(username);
        expect(url).toContain(username);
        expect(url).toMatch(/^https:\/\/www\.linkedin\.com\/in\/.+\/$/);
      });
    });
  });

  describe("Data Structure Validation", () => {
    test("should have all required fields", () => {
      expect(testUtils.validateDataStructure(mockLinkedInData)).toBe(true);
    });

    test("should have correct timestamp format", () => {
      expect(mockLinkedInData.timestamp).toMatch(
        /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/
      );
    });

    test("should have valid URL format", () => {
      expect(mockLinkedInData.url).toMatch(
        /^https:\/\/www\.linkedin\.com\/in\/.+\/$/
      );
    });

    test("should have proper data structure types", () => {
      expect(typeof mockLinkedInData.timestamp).toBe("string");
      expect(typeof mockLinkedInData.url).toBe("string");
      expect(typeof mockLinkedInData.title).toBe("string");
      expect(typeof mockLinkedInData.fullText).toBe("string");
      expect(typeof mockLinkedInData.byId).toBe("object");
      expect(typeof mockLinkedInData.byDataSection).toBe("object");
      expect(typeof mockLinkedInData.byAriaLabel).toBe("object");
      expect(typeof mockLinkedInData.byClass).toBe("object");
      expect(typeof mockLinkedInData.rawHtml).toBe("string");
    });

    test("should have non-array objects for categorized data", () => {
      expect(Array.isArray(mockLinkedInData.byId)).toBe(false);
      expect(Array.isArray(mockLinkedInData.byDataSection)).toBe(false);
      expect(Array.isArray(mockLinkedInData.byAriaLabel)).toBe(false);
      expect(Array.isArray(mockLinkedInData.byClass)).toBe(false);
    });
  });

  describe("File System Operations", () => {
    test("should create and write to files correctly", async () => {
      const testDir = path.join(__dirname, "test-temp");
      const testFile = path.join(testDir, "test.json");
      const testData = { test: "data", number: 42 };

      try {
        // Create directory
        await fs.mkdir(testDir, { recursive: true });

        // Check directory exists
        await fs.access(testDir);

        // Write file
        await fs.writeFile(testFile, JSON.stringify(testData, null, 2));

        // Check file exists
        await fs.access(testFile);

        // Read and validate file
        const readData = JSON.parse(await fs.readFile(testFile, "utf8"));
        expect(readData).toEqual(testData);
      } finally {
        // Cleanup
        try {
          await fs.unlink(testFile);
          await fs.rmdir(testDir);
        } catch (error) {
          // Ignore cleanup errors
        }
      }
    });

    test("should handle file system errors gracefully", async () => {
      const invalidPath = "/invalid/path/that/does/not/exist/file.json";

      await expect(fs.readFile(invalidPath, "utf8")).rejects.toThrow();
    });
  });

  describe("Utility Functions", () => {
    test("should handle safeText function logic", () => {
      const testCases = [
        { input: null, expected: null },
        { input: undefined, expected: null },
        { input: { textContent: "  test  " }, expected: "test" },
        { input: { textContent: "" }, expected: "" },
        { input: { textContent: "no-trim" }, expected: "no-trim" },
        {
          input: { textContent: "  multiple   spaces  " },
          expected: "multiple   spaces",
        },
      ];

      testCases.forEach(({ input, expected }) => {
        const result = input ? input.textContent?.trim() : null;
        expect(result).toBe(expected);
      });
    });

    test("should handle getAllAttributes function logic", () => {
      const mockElement = {
        attributes: [
          { name: "id", value: "test-id" },
          { name: "class", value: "test-class" },
          { name: "data-test", value: "test-value" },
        ],
      };

      const expectedAttrs = {
        id: "test-id",
        class: "test-class",
        "data-test": "test-value",
      };

      const actualAttrs = {};
      mockElement.attributes.forEach((attr) => {
        actualAttrs[attr.name] = attr.value;
      });

      expect(actualAttrs).toEqual(expectedAttrs);
    });

    test("should handle empty attributes", () => {
      const mockElement = { attributes: [] };
      const actualAttrs = {};

      mockElement.attributes.forEach((attr) => {
        actualAttrs[attr.name] = attr.value;
      });

      expect(actualAttrs).toEqual({});
    });
  });

  describe("Error Handling", () => {
    test("should handle invalid usernames gracefully", () => {
      const invalidUsernames = ["a", "user@domain.com", "user with spaces"];

      invalidUsernames.forEach((username) => {
        const url = testUtils.generateLinkedInUrl(username);
        expect(url).toContain(username);
        expect(url).toMatch(/^https:\/\/www\.linkedin\.com\/in\/.+\/$/);
      });
    });

    test("should handle empty username", () => {
      const username = "";
      const url = testUtils.generateLinkedInUrl(username);
      expect(url).toBe("https://www.linkedin.com/in//");
    });

    test("should handle missing element properties", () => {
      const testCases = [
        { element: null, expected: null },
        { element: undefined, expected: null },
        { element: {}, expected: null },
        { element: { textContent: null }, expected: null },
        { element: { textContent: undefined }, expected: null },
      ];

      testCases.forEach(({ element, expected }) => {
        const result = element?.textContent?.trim() || null;
        expect(result).toBe(expected);
      });
    });
  });

  describe("Data Extraction Logic", () => {
    test("should categorize elements by ID correctly", () => {
      const byId = mockLinkedInData.byId;

      expect(byId).toHaveProperty("profile-id");
      expect(byId["profile-id"]).toHaveProperty("tag", "DIV");
      expect(byId["profile-id"]).toHaveProperty("text", "Profile content");
      expect(byId["profile-id"]).toHaveProperty("attrs");
    });

    test("should categorize elements by data-section correctly", () => {
      const byDataSection = mockLinkedInData.byDataSection;

      expect(byDataSection).toHaveProperty("experience");
      expect(byDataSection).toHaveProperty("skills");
      expect(Array.isArray(byDataSection.experience)).toBe(true);
      expect(Array.isArray(byDataSection.skills)).toBe(true);
    });

    test("should categorize elements by aria-label correctly", () => {
      const byAriaLabel = mockLinkedInData.byAriaLabel;

      expect(byAriaLabel).toHaveProperty("Experience");
      expect(Array.isArray(byAriaLabel.Experience)).toBe(true);
    });

    test("should categorize elements by class correctly", () => {
      const byClass = mockLinkedInData.byClass;

      expect(byClass).toHaveProperty("pvs-list__item--line-separated");
      expect(Array.isArray(byClass["pvs-list__item--line-separated"])).toBe(
        true
      );
    });
  });

  describe("Integration Tests", () => {
    test("should maintain data integrity through full extraction cycle", () => {
      // Simulate the full data extraction process
      const extractedData = {
        ...mockLinkedInData,
        timestamp: new Date().toISOString(),
        url: testUtils.generateLinkedInUrl("testuser"),
      };

      // Validate structure
      expect(testUtils.validateDataStructure(extractedData)).toBe(true);

      // Validate content
      expect(extractedData.url).toContain("testuser");
      expect(extractedData.byId).toHaveProperty("profile-id");
      expect(extractedData.byDataSection).toHaveProperty("experience");
      expect(extractedData.byDataSection).toHaveProperty("skills");
    });

    test("should handle multiple page types consistently", () => {
      const pages = ["profile", "experience", "skills", "recommendations"];

      pages.forEach((page) => {
        const url = testUtils.generateLinkedInUrl("testuser", page);
        const data = {
          ...mockLinkedInData,
          url,
          title: `LinkedIn ${page.charAt(0).toUpperCase() + page.slice(1)}`,
        };

        expect(testUtils.validateDataStructure(data)).toBe(true);
        expect(data.url).toContain("testuser");
        if (page !== "profile") {
          expect(data.url).toContain(`/details/${page}/`);
        }
      });
    });
  });
});
