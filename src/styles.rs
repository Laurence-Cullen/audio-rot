// CSS Styles for Card-based Layout
pub const MAIN_CONTAINER_STYLE: &str = "
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 20px;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
";

pub const CARD_CONTAINER_STYLE: &str = "
    background: white;
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    padding: 24px;
    max-width: 350px;
    width: 100%;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
";

pub const TITLE_STYLE: &str = "
    margin: 0 0 20px 0;
    color: #2d3748;
    text-align: center;
    font-weight: 700;
    font-size: 20px;
    letter-spacing: -0.025em;
";

pub const FORM_GROUP_STYLE: &str = "
    margin-bottom: 16px;
";

pub const LABEL_STYLE: &str = "
    display: block;
    margin-bottom: 6px;
    color: #4a5568;
    font-weight: 500;
    font-size: 14px;
";

pub const INPUT_STYLE: &str = "
    width: 100%;
    padding: 12px;
    border: 2px solid #e2e8f0;
    border-radius: 8px;
    font-size: 16px;
    color: #2d3748;
    background: white;
    box-sizing: border-box;
    transition: border-color 0.2s ease;
    outline: none;
";

pub const BUTTON_STYLE: &str = "
    width: 100%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 8px;
    padding: 14px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    margin-top: 8px;
";

pub const STOP_BUTTON_STYLE: &str = "
    width: 100%;
    background: linear-gradient(135deg, #fc8181 0%, #f56565 100%);
    color: white;
    border: none;
    border-radius: 8px;
    padding: 14px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    margin-top: 8px;
";

pub const STATUS_ACTIVE_STYLE: &str = "
    text-align: center;
    font-weight: 600;
    padding: 12px;
    border-radius: 8px;
    margin: 16px 0 8px 0;
    background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
    color: white;
    font-size: 14px;
";

pub const STATUS_INACTIVE_STYLE: &str = "
    text-align: center;
    font-weight: 600;
    padding: 12px;
    border-radius: 8px;
    margin: 16px 0 8px 0;
    background: linear-gradient(135deg, #a0aec0 0%, #718096 100%);
    color: white;
    font-size: 14px;
";
