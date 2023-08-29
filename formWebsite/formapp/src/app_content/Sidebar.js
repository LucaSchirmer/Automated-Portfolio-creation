function Sidebar() {
  return (
    <sidenav className="sideNav"> 
        <h3 className="sideNavHeadline">
            Structure
        </h3>

        <ul className="sideNavUl">
            <li>
                <a href="#GeneralID">
                    General
                </a>
            </li>
            <li>
                <a href="#ProjectID">
                    Projects
                </a>
            </li>
            <li>
                <a href="#ServiceID">
                    Service
                </a>
            </li>
            <li>
                <a href="#CustomerID">
                    Customer
                </a>
            </li>
        </ul>
    </sidenav>
  );
}

export default Sidebar;
