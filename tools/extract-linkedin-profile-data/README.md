# extract-linkedin-profile-data

Comprehensive LinkedIn profile data extractor for any public profile, including all subpages (profile, experience, skills, recommendations). Uses Playwright for reliable browser automation.

## 🚀 Quick Start

### Prerequisites

- Node.js 16+
- npm or yarn

### Installation & Setup

```bash
# Install from GitHub
npm install github:dreamcodez/extract-linkedin-profile-data

# Or install locally
git clone <repository-url>
cd extract-linkedin-profile-data
npm install
npx playwright install chromium
```

### Basic Usage

```bash
# Extract a LinkedIn profile
node index.js <linkedin-username>

# Examples
node index.js matthewelder
node index.js johndoe
```

### Test Everything Works

```bash
npm test
```

## 📊 What You Get

The tool extracts data from 4 LinkedIn pages and saves to `dist/`:

- `linkedin-dump-profile.json` - Basic info, about, contact details
- `linkedin-dump-experience.json` - Employment history, positions, companies
- `linkedin-dump-skills.json` - Skills, endorsements, certifications
- `linkedin-dump-recommendations.json` - Testimonials and recommendations

### Sample Output

```json
{
  "timestamp": "2024-01-01T12:00:00.000Z",
  "url": "https://www.linkedin.com/in/username/",
  "title": "Page Title",
  "fullText": "All text content from the page",
  "byId": {
    "element-id": {
      "tag": "DIV",
      "text": "Content",
      "html": "<div>...</div>",
      "attrs": {}
    }
  },
  "byDataSection": {
    "experience": [
      {
        "tag": "SECTION",
        "text": "Job details",
        "html": "<section>...</section>",
        "attrs": {}
      }
    ]
  },
  "byAriaLabel": {},
  "byClass": {},
  "rawHtml": "<!DOCTYPE html><html>...</html>"
}
```

## 🧪 Testing

```bash
# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Run tests with coverage
npm run test:coverage

# Run tests with verbose output
npm run test:verbose

# Run tests with debug options
npm run test:debug

# Use Jest CLI directly
npx jest
npx jest --watch
npx jest --coverage
npx jest --verbose
```

**Test Status**: ✅ 24/24 tests passing

## 📚 Documentation

### For Users

- **This README** - Quick start and basic usage
- **[DEVELOPER.md](DEVELOPER.md)** - Detailed setup, architecture, and development workflow

### For Contributors

- **[FEATURES.md](FEATURES.md)** - Feature tracking (79 features documented)
- **[IMPROVEMENTS.md](IMPROVEMENTS.md)** - Future improvements roadmap

## 🎯 Current Status

| Metric                   | Value        | Status       |
| ------------------------ | ------------ | ------------ |
| **Features Implemented** | 27/79        | 34% Complete |
| **Test Cases**           | 24/24        | 100% Passing |
| **Documentation**        | Complete     | ✅           |
| **Code Quality**         | Professional | ✅           |

## 🚀 Next Steps

### Immediate (Next 2 Weeks)

1. Modular architecture refactoring
2. Parallel page extraction
3. Progress indicators
4. Data normalization
5. CSV export

### Short-term (Next Month)

1. RESTful API
2. Docker support
3. Batch processing
4. Configuration files
5. Error recovery

## 🔒 Legal & Ethical

- **Terms of Service**: Respects LinkedIn's terms of service
- **Public Data Only**: Only extracts publicly available information
- **Rate Limiting**: Built-in delays prevent excessive requests
- **Manual Login**: Requires manual authentication for security
- **Educational Use**: Intended for educational and research purposes

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/dreamcodez/extract-linkedin-profile-data/issues)
- **Documentation**: See [DEVELOPER.md](DEVELOPER.md) for detailed guides
- **Testing**: Run `npm test` to verify functionality

## 📄 License

BSD-3-Clause - See [LICENSE](LICENSE) file

---

**Note**: This tool is for educational and research purposes. Always respect LinkedIn's terms of service and use responsibly.
