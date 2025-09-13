// CSS Styles
pub const MAIN_CONTAINER_STYLE: &str = "
    min-height: 100vh;
    background-color: #36393f;
    color: #dcddde;
    font-family: Whitney, 'Helvetica Neue', Helvetica, Arial, sans-serif;
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
";

pub const FORM_CONTAINER_STYLE: &str = "
    background-color: #2f3136;
    padding: 30px;
    border-radius: 8px;
    box-shadow: 0 2px 10px 0 rgba(0,0,0,.2);
    display: flex;
    flex-direction: column;
    gap: 20px;
    min-width: 400px;
";

pub const TITLE_STYLE: &str = "margin: 0; color: #ffffff; text-align: center; font-weight: 600;";

pub const LABEL_STYLE: &str = "display: flex; flex-direction: column; gap: 8px;";

pub const LABEL_TEXT_STYLE: &str =
    "color: #b9bbbe; font-weight: 500; font-size: 14px; text-transform: uppercase;";

pub const INPUT_STYLE: &str = "
    background-color: #40444b;
    border: 1px solid #202225;
    border-radius: 4px;
    color: #dcddde;
    padding: 10px;
    font-size: 16px;
    outline: none;
";

pub const BUTTON_STYLE: &str = "
    background-color: #5865f2;
    color: #ffffff;
    border: none;
    border-radius: 4px;
    padding: 12px 20px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
    margin-top: 10px;
";

pub const STOP_BUTTON_STYLE: &str = "
    background-color: #ed4245;
    color: #ffffff;
    border: none;
    border-radius: 4px;
    padding: 12px 20px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
    margin-top: 10px;
";

pub const STATUS_ACTIVE_STYLE: &str = "
    text-align: center;
    font-weight: 500;
    padding: 10px;
    border-radius: 4px;
    margin-top: 10px;
    background-color: #3ba55c;
    color: #ffffff;
";

pub const STATUS_INACTIVE_STYLE: &str = "
    text-align: center;
    font-weight: 500;
    padding: 10px;
    border-radius: 4px;
    margin-top: 10px;
    background-color: #747f8d;
    color: #ffffff;
";
