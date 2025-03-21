// NavigationState conveniently exposes the state of the Navigation
// component to LeftNavigation and RightNavigation.
export type NavigationState = {
  isLeftOpen   : boolean
  isRightOpen  : boolean
  setLeftOpen  : (open: boolean) => void
  setRightOpen : (open: boolean) => void
}
