<script lang="ts">
  // Keeping the script empty as requested, bestie!
</script>

<div class="settings-container-glassy">
  <h1 class="page-title">Settings</h1>

  <section class="settings-section">
    <h2 class="section-title">Store Information</h2>
    <div class="setting-item">
      <label for="store-name">Store Name:</label>
      <input type="text" id="store-name" placeholder="Your Fabulous Store Name" />
    </div>
    <div class="setting-item">
      <label for="store-address">Address:</label>
      <input type="text" id="store-name" placeholder="123 Slay Street, City, Country" />
    </div>
    <div class="setting-item">
      <label for="store-phone">Phone:</label>
      <input type="tel" id="store-phone" placeholder="+123 456 7890" />
    </div>
    <div class="setting-item">
      <label for="store-email">Email:</label>
      <input type="email" id="store-email" placeholder="contact@yourstore.com" />
    </div>
  </section>

  <section class="settings-section">
    <h2 class="section-title">Receipt Customization</h2>
    <div class="setting-item">
      <label for="receipt-header">Header Message:</label>
      <input type="text" id="receipt-header" placeholder="Thank you for your purchase!" />
    </div>
    <div class="setting-item">
      <label for="receipt-footer">Footer Message:</label>
      <input type="text" id="receipt-footer" placeholder="Visit us again soon!" />
    </div>
    <div class="setting-item checkbox-item">
      <input type="checkbox" id="show-logo" />
      <label for="show-logo">Show Logo on Receipt</label>
    </div>
  </section>

  <section class="settings-section">
    <h2 class="section-title">Backup Options</h2>
    <div class="setting-item">
      <label for="backup-frequency">Backup Frequency:</label>
      <select id="backup-frequency">
        <option value="daily">Daily</option>
        <option value="weekly">Weekly</option>
        <option value="monthly">Monthly</option>
        <option value="manual">Manual</option>
      </select>
    </div>
    <div class="setting-item">
      <button class="action-button primary">Perform Backup Now</button>
      <button class="action-button secondary">Restore from Backup</button>
    </div>
  </section>

  <section class="settings-section">
    <h2 class="section-title">Printer Setup</h2>
    <div class="setting-item">
      <label for="printer-model">Printer Model:</label>
      <input type="text" id="printer-model" placeholder="Epson TM-T88VI" />
    </div>
    <div class="setting-item">
      <button class="action-button primary">Test Print</button>
      <button class="action-button secondary">Find Printers</button>
    </div>
  </section>

  <button class="save-button">Save All Settings</button>
</div>

<style>
  .settings-container-glassy {
    background: rgba(44, 44, 44, 0.5); /* Your glassy container background */
    border-radius: 40px;
    padding: 1.5rem;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
    width: 95%;
    color: #fff;
    display: flex;
    flex-direction: column;
    justify-content: center;
    overflow-y: auto;
    overflow-x: hidden;
    font-family: "goia regular";

    /* Glass effect */
    backdrop-filter: blur(10px) saturate(180%);
    -webkit-backdrop-filter: blur(10px) saturate(180%);
    border: 1px solid rgba(255, 255, 255, 0.15); /* Subtle border for definition */
/* Full height minus layout top/bottom padding */
    overflow-y: auto;
    &::-webkit-scrollbar-track {
      background: transparent;
    }

    &::-webkit-scrollbar-thumb {
      background-color: rgba(255, 255, 255, 0.2);
      border-radius: 10px;
      border: 2px solid transparent;
    }

    &::-webkit-scrollbar {
      width: 7px;
      height: 7px;
    }

    &::-webkit-scrollbar-thumb:hover {
      background-color: rgba(255, 255, 255, 0.4);
    }
  }

  .page-title {
    font-size: 2.5rem;
    font-weight: 700;
    color: #fff;
    margin-bottom: 2rem;
    text-align: center;
  }

  /* Each section within the main glassy container */
  .settings-section {
    padding-bottom: 2rem; /* Add padding at the bottom of each section */
    margin-bottom: 2rem; /* Space between sections */
    border-bottom: 1px solid rgba(255, 255, 255, 0.1); /* Subtle separator */
  }

  /* Remove the last border for cleaner look */
  .settings-section:last-of-type {
    border-bottom: none;
    margin-bottom: 0; /* Remove margin from the last section */
    padding-bottom: 0;
  }

  .section-title {
    font-size: 1.8rem;
    font-weight: 600;
    color: #fff;
    margin-bottom: 1.5rem;
    padding-bottom: 0.5rem;
  }

  .setting-item {
    display: flex;
    flex-direction: column;
    margin-bottom: 1rem;
  }


  .setting-item label {
    font-size: 0.95rem;
    color: #ccc;
    margin-bottom: 0.4rem;
    font-weight: 500;
  }

  .setting-item input[type="text"],
  .setting-item input[type="tel"],
  .setting-item input[type="email"],
  .setting-item select {
    width: 97%;
    padding: 0.8rem 1rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 40px;
    background-color: rgba(0, 0, 0, 0.3);
    color: #fff;
    font-size: 1rem;
    outline: none;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .setting-item input::placeholder {
    color: rgba(255, 255, 255, 0.5);
    font-family: "goia regular";
  }

  .setting-item input:focus,
  .setting-item select:focus {
    border-color: #ffffff;
  }


  .setting-item select {
    appearance: none;
    -webkit-appearance: none;
    background-image: url('data:image/svg+xml;charset=UTF-8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="white"><path d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" fill-rule="evenodd"></path></svg>');
    background-repeat: no-repeat;
    background-position: right 0.7rem center;
    background-size: 1.2em auto;
    cursor: pointer;
    width: auto;
  }

  .checkbox-item {
    flex-direction: row;
    align-items: center;
  }

  .checkbox-item input[type="checkbox"] {
    width: auto;
    margin-right: 0.5rem;
    transform: scale(1.2);
  }

  .action-button {
    background-color: rgba(0, 0, 0, 0.3);
    color: #fff;
    border: 1px solid rgba(255, 255, 255, 0.2);
    padding: 0.8rem 1.5rem;
    margin-bottom: 0.5rem;
    border: none;
    border-radius: 40px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 600;
    transition: background-color 0.2s ease, transform 0.1s ease;
    font-family: "goia regular";
  }


  .action-button:hover {
    background-color: rgba(255, 255, 255, 0.2);
  }

  .save-button {
    display: block;
    width: fit-content;
    margin: 2rem auto 0;
    padding: 1rem 3rem;
    background-color: #A020F0;
    box-shadow: 0 4px 15px rgba(160, 32, 240, 0.4);
    color: #fff;
    border: none;
    border-radius: 40px;
    font-size: 1.2rem;
    font-weight: 700;
    cursor: pointer;
    transition: background-color 0.3s ease, transform 0.2s ease;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
    font-family: "goia regular";
  }

  .save-button:hover {
    background-color: #8a00d9;
    border-color: #a020f0;
    box-shadow: 0 4px 20px rgba(160, 32, 240, 0.6);
  }
</style>